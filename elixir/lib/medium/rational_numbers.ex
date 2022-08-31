defmodule RationalNumbers do
  @type rational :: {integer, integer}

  @spec add(r1 :: rational, r2 :: rational) :: rational
  def add({n1, d1}, {n2, d2}) do
    {n1 * d2 + n2 * d1, d1 * d2}
    |> reduce()
  end

  @spec subtract(r1 :: rational, r2 :: rational) :: rational
  def subtract({n1, d1}, {n2, d2}) do
    {n1 * d2 - n2 * d1, d1 * d2}
    |> reduce()
  end

  @spec multiply(r1 :: rational, r2 :: rational) :: rational
  def multiply({n1, d1}, {n2, d2}) do
    {n1 * n2, d1 * d2}
    |> reduce()
  end

  @spec divide_by(num :: rational, den :: rational) :: rational
  def divide_by({n1, d1}, {n2, d2}) do
    {n1 * d2, d1 * n2}
    |> reduce()
  end

  @spec abs(r :: rational) :: rational
  def abs({n, d}), do: {Kernel.abs(n), Kernel.abs(d)}

  @spec pow_rational(r :: rational, pow :: integer) :: rational
  def pow_rational({n, d}, pow) when pow >= 0, do: {n ** pow, d ** pow}
  def pow_rational({n, d}, pow), do: {d ** -pow, n ** -pow}

  @spec pow_real(x :: integer, r :: rational) :: float
  def pow_real(x, {n, d}), do: x ** (n / d)

  @spec reduce(r :: rational) :: rational
  def reduce({n, d}) do
    gcd = Integer.gcd(n, d)
    numerator = div(n, gcd)
    denominator = div(d, gcd)

    case {numerator, denominator} do
      {numerator, denominator} when denominator < 0 -> {-numerator, -denominator}
      _ -> {numerator, denominator}
    end
  end
end
