defmodule Prime do
  @doc """
  Generates the nth prime.
  """
  def nth(0), do: raise("There is no zeroth prime")
  def nth(count) do
    iter_primes(0, count)
  end

  defp iter_primes(maybe_prime, 0), do: maybe_prime
  defp iter_primes(maybe_prime, count) do
    if prime?(maybe_prime + 1) do
      iter_primes(maybe_prime + 1, count - 1)
    else
      iter_primes(maybe_prime + 1, count)
    end
  end

  defp prime?(2), do: true
  defp prime?(num) do
    Enum.all?(2..(num - 1), &(rem(num, &1) != 0))
  end
end
