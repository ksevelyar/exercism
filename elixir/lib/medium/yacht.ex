defmodule Yacht do
  @type category ::
          :ones
          | :twos
          | :threes
          | :fours
          | :fives
          | :sixes
          | :full_house
          | :four_of_a_kind
          | :little_straight
          | :big_straight
          | :choice
          | :yacht

  @doc """
  Calculate the score of 5 dice using the given category's scoring method.
  """
  @spec score(category :: category(), dice :: [integer]) :: integer
  def score(:ones, dice), do: Enum.count(dice, &(&1 == 1))
  def score(:twos, dice), do: Enum.count(dice, &(&1 == 2)) * 2
  def score(:threes, dice), do: Enum.count(dice, &(&1 == 3)) * 3
  def score(:fours, dice), do: Enum.count(dice, &(&1 == 4)) * 4
  def score(:fives, dice), do: Enum.count(dice, &(&1 == 5)) * 5
  def score(:sixes, dice), do: Enum.count(dice, &(&1 == 6)) * 6

  def score(:full_house, dice) do
    counts = dice |> counts() |> Map.values() |> Enum.sort()

    case counts do
      [2, 3] -> Enum.sum(dice)
      _ -> 0
    end
  end

  def score(:four_of_a_kind, dice) do
    kind = dice |> counts() |> Enum.find(fn {_key, val} -> val >= 4 end)

    case kind do
      {num, _count} -> num * 4
      nil -> 0
    end
  end

  def score(:little_straight, dice) do
    if Enum.sort(dice) == [1, 2, 3, 4, 5], do: 30, else: 0
  end

  def score(:big_straight, dice) do
    if Enum.sort(dice) == [2, 3, 4, 5, 6], do: 30, else: 0
  end

  def score(:choice, dice), do: Enum.sum(dice)

  def score(:yacht, [head | rest]) do
    if Enum.all?(rest, &(&1 == head)), do: 50, else: 0
  end

  defp counts(dice) do
    Enum.reduce(dice, %{}, fn num, counts ->
      Map.update(counts, num, 1, &(&1 + 1))
    end)
  end
end
