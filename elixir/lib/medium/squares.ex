defmodule Squares do
  def sum_of_squares(number) do
    1..number |> Enum.map(&(&1 ** 2)) |> Enum.sum()
  end

  def square_of_sum(number) do
    Enum.sum(1..number) ** 2
  end

  def difference(number) do
    square_of_sum(number) - sum_of_squares(number)
  end
end
