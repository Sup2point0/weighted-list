'''
Records speedtest metrics.
'''

import pathlib
from textwrap import dedent


def record(data: dict):
  '''Record metrics results to a Markdown file.'''

  start = data["_meta"]["start"]
  stop = data["_meta"]["stop"]
  runtime = stop - start

  path = pathlib.Path().absolute().parent
  name = start.strftime("%y-%m-%d.%H-%M-%S")

  content = _generate_record_(data)

  with open(f"{path}{time}.md", "w") as file:
    file.write(content)


def _generate_record_(data: dict) -> str:
  '''Generate the content to write to a record.'''

  return (
    dedent(f'''
      # Performance Metrics
  
      <table>
        <tr>
          <td> test started </td>
          <td> {start.strftime("%H:%M.%S")} </td>
        </tr>
        <tr>
          <td> test finished </td>
          <td> {start.strftime("%H:%M.%S")} </td>
        </tr>
        <tr>
          <td> total runtime </td>
          <td> {runtime.strftime("%H:%M.%S")} </td>
        </tr>
      </table>
    ''').strip()
    +
    "\n".join(
      dedent(f'''
      ## `{func}`
  
      | results["metric"] | `list` | `WeightedList` | `FrozenWeightedList` |
      | :---------------- | :----- | :------------- | :------------------- |
      {"\n".join(
        f"| `{test}` | `{test['list']}` " +
        f"| `{test.get('wl')}` | `{test.get('fwl', '–')}` |"
      for test in results)}
      ''').strip()
      for func, results in data.items()
    ).strip()
  )
