defmodule SumOfMultiples do
  @doc """
  Adds up all numbers from 1 to a given end number that are multiples of the factors provided.
  """
  @spec to(non_neg_integer, [non_neg_integer]) :: non_neg_integer
  def to(limit, factors) do
    Enum.reduce(1..(limit - 1), 0, fn num, acc ->
      multiple?(factors, num) && acc + num || acc
    end)
  end

  defp multiple?(factors, num) do
    Enum.any?(factors, fn factor ->
      case factor do
        0 -> false
        _ -> rem(num, factor) == 0
      end
    end)
  end
end
