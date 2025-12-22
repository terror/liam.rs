I recently wrote and distributed a [Python package](https://pypi.org/project/pqg/){target="_blank"}
for a project, and I wanted a [web interface](https://dislmcgill.github.io/pandas-query-generator/){target="_blank"} to go with it. Naturally, I chose [React](https://react.dev/){target="_blank"}
to build it, but I had never called Python from a React app before, so I had to figure things out.

After a quick search, I stumbled across [Pyodide](https://pyodide.org/en/stable/){target="_blank"}, which is a port of
[CPython](https://en.wikipedia.org/wiki/CPython?useskin=vector){target="_blank"} to [WebAssembly](https://webassembly.org/){target="_blank"}/[Emscripten](https://emscripten.org/){target="_blank"}. It allows users to install and use any pure
Python package with a wheel on [PyPI](https://pypi.org/){target="_blank"} via [micropip](https://micropip.pyodide.org/en/stable/project/api.html){target="_blank"}. This
sounded like something that could be useful, so I decided to try it out.

Since my package was already distributed, all I needed to do, according to the
Pyodide [documentation](https://pyodide.org/en/stable/usage/loading-packages.html){target="_blank"}, was:

1. `npm install pyodide`
2. Install my package with micropip
3. Run arbitrary Python code using my package with Pyodide's [API](https://pyodide.org/en/stable/usage/api/js-api.html#pyodide.runPythonAsync){target="_blank"}

Turns out this worked for my use case, let's dive deeper into each step!

### I. Installation

This is straightforward. I used [bun](https://bun.sh/){target="_blank"} as my package manager, so all I had
to do was run `bun add pyodide`. Other package managers should follow a similar
command. You can check out the official Pyodide package [on the npm registry](https://www.npmjs.com/package/pyodide){target="_blank"}.

### II. Loading the package

Getting Pyodide up and running involves a few steps that need to happen in the right order. First, we need to load the core Pyodide runtime from a [CDN](https://en.wikipedia.org/wiki/Content_delivery_network){target="_blank"}. Then we can use Pyodide's package manager, micropip, to install our Python package. This is similar to how you might use [pip](https://pip.pypa.io/en/stable/installation/){target="_blank"} in a regular Python environment, but it's all happening in the browser.

One nice thing about Pyodide's CDN distribution is that it includes many common Python packages pre-built and ready to use. However, for packages that aren't included by default (like my `pqg` package), we need to install them explicitly using micropip. Here's the complete initialization sequence:

```typescript
import { PyodideInterface, loadPyodide } from 'pyodide';

// Initialize Pyodide with the specified CDN URL
// This loads the core Python runtime environment into the browser
const client = await loadPyodide({
  indexURL: 'https://cdn.jsdelivr.net/pyodide/v0.26.3/full',
});

// Load the micropip package, which is Pyodide's package installer
// This is similar to pip for regular Python, but works in the browser
await pyodide.loadPackage('micropip');

// Import the micropip module into the JavaScript environment
// This allows us to use micropip's functions from JavaScript
const micropip = pyodide.pyimport('micropip');

// Use micropip to install the 'pqg' package from PyPI
// This downloads and installs the pure Python package in the browser environment
await micropip.install('pqg');
```

It's worth noting that this initialization process can take a few seconds, especially on slower connections. The Pyodide runtime itself is about [6MB](https://pyodide.org/en/stable/project/roadmap.html#reducing-download-sizes-and-initialization-times){target="_blank"}, and any additional packages you install will add to that. In my case, I found that showing a loading indicator during this initialization was essential for a good user experience.

Also, because these operations are asynchronous, you need to be careful about when you start trying to execute Python code. I learned the hard way that trying to run code before Pyodide is fully initialized will fail. This is one of the reasons I eventually moved all of this into a web worker, which we'll discuss later.

### III. Running code

Now that we have Pyodide and our package loaded, we need to actually run some Python code. In my case, I needed to generate pandas queries based on user-defined schemas and settings. One early challenge I encountered was that Pyodide doesn't support multiprocessing due to browser limitations - I had to explicitly disable it in my package's configuration to ensure compatibility. Here's how I structured the code generation (abridged):

```typescript
// Generate Python code that creates and configures the query generator
const generateQueryGenerationCode = (schema: string, settings: Settings) => `
  import json

  from pqg import Generator, QueryStructure, Schema, QueryPool, GenerateOptions

  # Parse the schema from JSON
  schema = Schema.from_dict(json.loads('''${schema}'''))

  # Configure query structure based on user settings
  query_structure = QueryStructure(
    groupby_aggregation_probability=${settings.groupbyProbability},
    max_groupby_columns=${settings.maxGroupbyColumns},
    # ... other settings
  )

  generator = Generator(schema, query_structure)

  generate_options = GenerateOptions(
    ensure_non_empty=${settings.enforceNonEmptyResults ? 'True' : 'False'},
    multi_line=${settings.multiLine ? 'True' : 'False'},
    multi_processing=False,
    num_queries=${settings.numQueries}
  )

  query_pool = generator.generate(generate_options)

  # Return results as JSON
  json.dumps({
    'queries': [str(query) for query in query_pool],
    # ... other keys
  })
`;

// Execute the code using Pyodide
const response = await client.runPythonAsync(generateQueryGenerationCode(schema, settings));
```

The key here is that we're generating a Python code string that will be executed by Pyodide. We interpolate JavaScript values (the schema and settings) into the Python code string. This allows us to pass configuration from our React UI to the Python runtime.

One important detail is that we need to handle the data serialization carefully. Since we're crossing the JavaScript-Python boundary, we use JSON as our data format. The Python code parses the JSON schema and returns its results as JSON, which can then be parsed back into JavaScript objects.

### IV. Inside a web worker

When I first implemented this, I quickly realized that loading and running Python in the main thread wasn't ideal - it would freeze the UI during initialization and execution. The solution was to move all Pyodide-related code into a [web worker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers){target="_blank"}.

Here's how I built the worker interface (abridged):

```typescript
import { useEffect, useRef, useState } from 'react';

import { useToast } from './use-toast';

type WorkerStatus = 'idle' | 'loading' | 'ready' | 'error';

type WorkerResponse =
  | { type: 'status'; payload: WorkerStatus }
  | { type: 'error'; payload: string }
  | { type: 'result'; payload: string };

export const usePyodideWorker = () => {
  const [status, setStatus] = useState<WorkerStatus>('idle');

  const workerRef = useRef<Worker>();

  const { toast } = useToast();

  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../workers/pyodide.worker.ts', import.meta.url),
      { type: 'module' }
    );

    workerRef.current.onmessage = (event: MessageEvent<WorkerResponse>) => {
      const { type, payload } = event.data;

      switch (type) {
        case 'status':
          setStatus(payload);
          break;
        case 'error':
          // ...
          setStatus('error');
          break;
      }
    };

    workerRef.current.postMessage({ type: 'init' });

    return () => {
      workerRef.current?.terminate();
    };
  }, [toast]);

  const runPython = async <T>(code: string): Promise<T> => {
    return new Promise((resolve, reject) => {
      // ...
    });
  };

  return {
    error: status === 'error',
    loading: status === 'loading',
    ready: status === 'ready',
    runPython,
  };
};
```

I created this as a React [hook](https://react.dev/learn#using-hooks){target="_blank"} that manages the worker lifecycle. The hook creates the worker when the component mounts and provides a [`runPython`](https://github.com/DISLMcGill/pandas-query-generator/blob/303251c5f4ae4bdcdc945caac0c0f21a22fda56f/www/src/hooks/use-pyodide-worker.ts#L52){target="_blank"} function that components can use to execute Python code. The status updates help the UI show loading states while Pyodide initializes or executes code.

The worker implementation itself is relatively straightforward:

```typescript
import { PyodideInterface, loadPyodide } from 'pyodide';

let pyodide: PyodideInterface;

type WorkerMessage =
  | { type: 'init' }
  | { type: 'run'; payload: { code: string } };

type WorkerResponse =
  | { type: 'status'; payload: 'loading' | 'ready' }
  | { type: 'error'; payload: string }
  | { type: 'result'; payload: string };

self.onmessage = async (event: MessageEvent<WorkerMessage>) => {
  const { type } = event.data;

  try {
    switch (type) {
      case 'init':
        if (!pyodide) {
          self.postMessage({
            type: 'status',
            payload: 'loading',
          } satisfies WorkerResponse);

          pyodide = await loadPyodide({
            indexURL: 'https://cdn.jsdelivr.net/pyodide/v0.26.3/full',
          });

          await pyodide.loadPackage('micropip');

          const micropip = pyodide.pyimport('micropip');
          await micropip.install('pqg');

          self.postMessage({
            type: 'status',
            payload: 'ready',
          } satisfies WorkerResponse);
        }
        break;
      case 'run':
        if (!pyodide) throw new Error('Pyodide not initialized');

        self.postMessage({
          type: 'result',
          payload: await pyodide.runPythonAsync(event.data.payload.code),
        } satisfies WorkerResponse);

        break;
      default:
        throw new Error(`Unknown message type: ${type satisfies never}`);
    }
  } catch (error) {
    self.postMessage({
      type: 'error',
      payload: error instanceof Error ? error.message : 'Unknown error',
    } satisfies WorkerResponse);
  }
};
```

The worker maintains the Pyodide instance and handles two types of messages: initialization and code execution. This keeps all the heavy lifting off the main thread, ensuring the UI stays responsive.

One thing to note is that web workers have their own [global scope](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope){target="_blank"}, so the Pyodide instance and any installed packages persist between executions. This is actually beneficial as it means we only need to load Pyodide and install packages once, rather than for each query generation.

### Fin

Building this interface turned out to be surprisingly straightforward thanks to Pyodide. The ability to run Python code directly in the browser, without needing a backend server, makes for a great developer experience. The combination of React for the UI, Pyodide for Python execution, and web workers for performance created a smooth, interactive interface for my Python package.

While there are [some limitations](https://pyodide.org/en/stable/usage/wasm-constraints.html){target="_blank"} - Pyodide can only run pure Python packages, and the initial load time can be significant - it's a powerful tool for the right use case. In my case, it allowed me to create a web interface for my Python package with minimal overhead and deployment complexity.

The full code for the implementation can be [found on GitHub](https://github.com/DISLMcGill/pandas-query-generator/tree/master/www){target="_blank"}.
