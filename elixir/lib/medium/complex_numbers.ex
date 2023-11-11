defmodule ComplexNumbers do
  @typedoc """
  In this module, complex numbers are represented as a tuple-pair containing the real and
  imaginary parts.
  For example, the real number `1` is `{1, 0}`, the imaginary number `i` is `{0, 1}` and
  the complex number `4+3i` is `{4, 3}'.
  """
  @type complex :: {float, float}

  @doc """
  Return the real part of a complex number
  """
  @spec real(a :: complex) :: float
  def real({real, _}), do: real

  @doc """
  Return the imaginary part of a complex number
  """
  @spec imaginary({{a :: complex}}) :: float
  def imaginary({_, imaginary}), do: imaginary

  @doc """
  Multiply two complex numbers, or a real and a complex number
  """
  @spec mul(a :: complex | float, b :: complex | float) :: complex
  def mul({a, b}, {c, d}), do: {a * c - b * d, b * c + a * d}
  def mul({a, b}, r), do: {a * r, b * r}
  def mul(r, {a, b}), do: {a * r, b * r}

  @doc """
  Add two complex numbers, or a real and a complex number
  """
  @spec add(a :: complex | float, b :: complex | float) :: complex
  def add({a, b}, {c, d}), do: {a + c, b + d}
  def add(r, {c, d}), do: {r + c, d}
  def add({c, d}, r), do: {r + c, d}

  @doc """
  Subtract two complex numbers, or a real and a complex number
  """
  @spec sub(a :: complex | float, b :: complex | float) :: complex
  def sub({a, b}, {c, d}), do: {a - c, b - d}
  def sub(r, {c, d}), do: {r - c, -d}
  def sub({c, d}, r), do: {c - r, d}

  @doc """
  Divide two complex numbers, or a real and a complex number
  """
  @spec div(a :: complex | float, b :: complex | float) :: complex
  def div({a, b}, {c, d}), do: {(a * c + b * d) / (c ** 2 + d ** 2), (b * c - a * d) / (c ** 2 + d ** 2)}
  def div(r, {c, d}), do: {r * c / (c ** 2 + d ** 2), -r * c / (c ** 2 + d ** 2)}
  def div({c, d}, r), do: {c / r, d / r}

  @doc """
  Absolute value of a complex number
  """
  @spec abs(a :: complex) :: float
  def abs({a, b}), do: :math.sqrt(a ** 2 + b ** 2)

  @doc """
  Conjugate of a complex number
  """
  @spec conjugate(a :: complex) :: complex
  def conjugate({a, b}), do: {a, -b}

  @doc """
  Exponential of a complex number
  """
  @spec exp(a :: complex) :: complex
  def exp({a, b}) do
    {:math.exp(a) * :math.cos(b), :math.exp(a) * :math.sin(b)}
  end
end
