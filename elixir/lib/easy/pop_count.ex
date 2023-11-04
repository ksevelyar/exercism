defmodule PopCount do
  @doc """
  Given the number, count the number of eggs.
  """
  import Bitwise

  @spec egg_count(number :: integer()) :: non_neg_integer()
  def egg_count(number) do
    egg_count_with_acc(number, 1, 0)
  end

  defp egg_count_with_acc(number, pow, eggs) do
    cond do
      2 ** pow > number ->
        eggs

      2 ** pow &&& number == 0 ->
        egg_count_with_acc(number, pow + 1, eggs)

      true ->
        egg_count_with_acc(number, pow + 1, eggs + 1)
    end
  end
end
