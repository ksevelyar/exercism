defmodule ResistorColorTrio do
  @doc """
  Calculate the resistance value in ohm or kiloohm from resistor colors
  """

  @color_values %{
    black: 0,
    brown: 1,
    red: 2,
    orange: 3,
    yellow: 4,
    green: 5,
    blue: 6,
    violet: 7,
    grey: 8,
    white: 9
  }

  @spec label(colors :: [atom]) :: {number, :ohms | :kiloohms}
  def label(colors) do
    [first_digit, second_digit, powers_of_10] =
      Enum.map(colors, fn color -> @color_values[color] end)

    (Integer.undigits([first_digit, second_digit]) * 10 ** powers_of_10)
    |> parse_value()
  end

  defp parse_value(value) when div(value, 1000) > 0, do: {div(value, 1000), :kiloohms}
  defp parse_value(value), do: {value, :ohms}
end
