'''
Records speedtest metrics.
'''

import pathlib
from textwrap import dedent

from .speedtests import TestMetrics


def record(data: TestMetrics):
  '''Record metrics results to a Markdown file.'''

  root = pathlib.Path().absolute().parent
  time = data.meta["start"].strftime("%y-%m-%d.%H-%M-%S")
  path = root.joinpath("metrics/results", time)

  content = _generate_record_(data)

  with open(f"{path}.md", "w") as file:
    file.write(content)


def _generate_record_(data: TestMetrics) -> str:
  '''Generate the content to write to a record.'''

  return (
    dedent(f'''
      # Performance Metrics
  
      <table>
        <tr>
          <td> test started </td>
          <td> {data.meta["start"].strftime("%H:%M.%S")} </td>
        </tr>
        <tr>
          <td> test finished </td>
          <td> {data.meta["stop"].strftime("%H:%M.%S")} </td>
        </tr>
        <tr>
          <td> total runtime </td>
          <td> {round(data.meta["delta"].total_seconds(), 5)} s </td>
        </tr>
      </table>
    ''').strip()
    +
    "\n\n"
    +
    "\n".join(
      dedent(f'''
        ## `{func[5:]}`
    
        | {results["_metric"]} | `list` | `WeightedList` | `FrozenWeightedList` |
        | :{"-" * (len(results["_metric"]) - 1)} | :----- | :------------- | :------------------- |
{"\n".join(
          (
            f"        | `{test}` | `{round(result['list'], 5)}` "
            f"| `{round(result.get('wl', 0), 5)}` "
            f"| `{round(result.get('fwl', 0), 5)}` |"
          )
          for test, result in results.items()
          if not str(test).startswith("_")
        )}
      ''').strip()
      for func, results in data.tests.items()
    ).strip()
  )
