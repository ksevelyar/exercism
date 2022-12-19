defmodule ResistorColorDuo do
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

  @doc """
  Calculate a resistance value from two colors
  """
  @spec value(colors :: [atom]) :: integer
  def value([first_color, second_color | _]) do
    [first_color, second_color] |> Enum.map(&@color_values[&1]) |> Integer.undigits()
  end
end
