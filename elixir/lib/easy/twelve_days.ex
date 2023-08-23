defmodule TwelveDays do
  @doc """
  Given a `number`, return the song's verse for that specific day, including
  all gifts for previous days in the same line.
  """
  @spec verse(number :: integer) :: String.t()
  def verse(1), do: "#{intro("first")} #{present(1)}"
  def verse(2), do: "#{intro("second")} #{presents(2)}"
  def verse(3), do: "#{intro("third")} #{presents(3)}"
  def verse(4), do: "#{intro("fourth")} #{presents(4)}"
  def verse(5), do: "#{intro("fifth")} #{presents(5)}"
  def verse(6), do: "#{intro("sixth")} #{presents(6)}"
  def verse(7), do: "#{intro("seventh")} #{presents(7)}"
  def verse(8), do: "#{intro("eighth")} #{presents(8)}"
  def verse(9), do: "#{intro("ninth")} #{presents(9)}"
  def verse(10), do: "#{intro("tenth")} #{presents(10)}"
  def verse(11), do: "#{intro("eleventh")} #{presents(11)}"
  def verse(12), do: "#{intro("twelfth")} #{presents(12)}"

  defp intro(day), do: "On the #{day} day of Christmas my true love gave to me:"

  defp presents(n) do
    presents = n..2 |> Enum.map(&present/1) |> Enum.join(", ")

    "#{presents}, and #{present(1)}"
  end

  defp present(1), do: "a Partridge in a Pear Tree."
  defp present(2), do: "two Turtle Doves"
  defp present(3), do: "three French Hens"
  defp present(4), do: "four Calling Birds"
  defp present(5), do: "five Gold Rings"
  defp present(6), do: "six Geese-a-Laying"
  defp present(7), do: "seven Swans-a-Swimming"
  defp present(8), do: "eight Maids-a-Milking"
  defp present(9), do: "nine Ladies Dancing"
  defp present(10), do: "ten Lords-a-Leaping"
  defp present(11), do: "eleven Pipers Piping"
  defp present(12), do: "twelve Drummers Drumming"

  @doc """
  Given a `starting_verse` and an `ending_verse`, return the verses for each
  included day, one per line.
  """
  @spec verses(starting_verse :: integer, ending_verse :: integer) :: String.t()
  def verses(starting_verse, ending_verse) do
    starting_verse..ending_verse |> Stream.map(&verse/1) |> Enum.join("\n")
  end

  @doc """
  Sing all 12 verses, in order, one verse per line.
  """
  @spec sing() :: String.t()
  def sing, do: verses(1, 12)
end
