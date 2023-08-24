defmodule PrimeFactors do
  @doc """
  Compute the prime factors for 'number'.

  The prime factors are prime numbers that when multiplied give the desired
  number.

  The prime factors of 'number' will be ordered lowest to highest.
  """
  @spec factors_for(pos_integer) :: [pos_integer]
  def factors_for(number), do: factors_with_acc(number, [])

  defp factors_with_acc(1, factors), do: Enum.reverse(factors)
  defp factors_with_acc(number, factors) do
    factor = Enum.find(2..number, &(rem(number, &1) == 0))

    factors_with_acc(div(number, factor), [factor | factors])
  end
end
