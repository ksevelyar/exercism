defmodule BottleSong do
  @moduledoc """
  Handles lyrics of the popular children song: Ten Green Bottles
  """

  @numbers %{
    10 => "Ten",
    9 => "Nine",
    8 => "Eight",
    7 => "Seven",
    6 => "Six",
    5 => "Five",
    4 => "Four",
    3 => "Three",
    2 => "Two",
    1 => "One",
    0 => "no"
  }

  @spec recite(pos_integer, pos_integer) :: String.t()
  def recite(start_bottle, take_down) do
    start_bottle..(start_bottle - take_down + 1)
    |> Enum.map(&verse/1)
    |> Enum.join("\n\n")
  end

  defp verse(n) do
    template({@numbers[n], String.downcase(@numbers[n - 1])})
  end

  defp template({left, left_minus_one}) do
    bottles_left_form = if left == "One", do: "bottle", else: "bottles"
    bottles_left_minus_one_form = if left_minus_one == "one", do: "bottle", else: "bottles"

    """
    #{left} green #{bottles_left_form} hanging on the wall,
    #{left} green #{bottles_left_form} hanging on the wall,
    And if one green bottle should accidentally fall,
    There'll be #{left_minus_one} green #{bottles_left_minus_one_form} hanging on the wall.\
    """
  end
end
