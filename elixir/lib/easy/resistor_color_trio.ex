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
    ohms = colors_to_ohms(colors)
    format_output(ohms, div(ohms, 1000))
  end

  defp colors_to_ohms(colors) do
    [first_digit, second_digit, powers_of_10] =
      Enum.map(colors, fn color -> @color_values[color] end)

    Integer.undigits([first_digit, second_digit]) * 10 ** powers_of_10
  end

  defp format_output(_ohms, kiloohms) when kiloohms > 0, do: {kiloohms, :kiloohms}
  defp format_output(ohms, _kiloohms), do: {ohms, :ohms}
end
