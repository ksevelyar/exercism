defmodule Sieve do
  @min_prime 2

  def primes_to(limit) do
    sieve(Enum.to_list(@min_prime..limit), @min_prime, limit)
  end

  defp sieve(_numbers, _prime, limit) when limit < @min_prime, do: []

  defp sieve(numbers, nil, _limit), do: numbers

  defp sieve(numbers, prime, limit) do
    composites = Stream.map(2..limit, &(&1 * prime)) |> Enum.take_while(&(&1 <= limit))
    next_prime = Enum.find(numbers, &(&1 > prime))

    sieve(numbers -- composites, next_prime, limit)
  end
end
