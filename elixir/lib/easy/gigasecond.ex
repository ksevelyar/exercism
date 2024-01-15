defmodule Gigasecond do
  @doc """
  Calculate a date one billion seconds after an input date.
  """
  @spec from({{pos_integer, pos_integer, pos_integer}, {pos_integer, pos_integer, pos_integer}}) ::
          {{pos_integer, pos_integer, pos_integer}, {pos_integer, pos_integer, pos_integer}}
  def from({{year, month, day}, {hours, minutes, seconds}}) do
    from = %NaiveDateTime{
      year: year,
      month: month,
      day: day,
      hour: hours,
      minute: minutes,
      second: seconds
    }

    result = NaiveDateTime.add(from, 1_000_000_000)

    {{result.year, result.month, result.day}, {result.hour, result.minute, result.second}}
  end
end
