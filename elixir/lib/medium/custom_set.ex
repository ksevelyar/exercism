defmodule CustomSet do
  defstruct map: %{}
  @opaque t :: %__MODULE__{map: map}

  @spec new(Enum.t()) :: t
  def new(enumerable) do
    map = Enum.into(enumerable, %{}, fn item -> {item, true} end)

    %__MODULE__{map: map}
  end

  @spec empty?(t) :: boolean
  def empty?(%__MODULE__{map: map}) when map == %{}, do: true
  def empty?(%__MODULE__{}), do: false

  @spec contains?(t, any) :: boolean
  def contains?(%__MODULE__{map: map}, element) do
    Map.has_key?(map, element)
  end

  @spec subset?(t, t) :: boolean
  def subset?(custom_set_1, custom_set_2) do
    Enum.all?(custom_set_1.map, fn {key, _} -> custom_set_2.map[key] end)
  end

  @spec disjoint?(t, t) :: boolean
  def disjoint?(custom_set_1, custom_set_2) do
    Enum.all?(custom_set_1.map, fn {key, _} -> !custom_set_2.map[key] end)
  end

  @spec equal?(t, t) :: boolean
  def equal?(custom_set_1, custom_set_2) do
    custom_set_1.map == custom_set_2.map
  end

  @spec add(t, any) :: t
  def add(custom_set, element) do
    put_in(custom_set.map[element], true)
  end

  @spec intersection(t, t) :: t
  def intersection(custom_set_1, custom_set_2) do
    elements = Enum.filter(custom_set_1.map, fn {key, _} -> custom_set_2.map[key] end)

    %__MODULE__{map: Map.new(elements)}
  end

  @spec difference(t, t) :: t
  def difference(custom_set_1, custom_set_2) do
    elements = Enum.filter(custom_set_1.map, fn {key, _} -> !custom_set_2.map[key] end)

    %__MODULE__{map: Map.new(elements)}
  end

  @spec union(t, t) :: t
  def union(custom_set_1, custom_set_2) do
    map = Map.merge(custom_set_1.map, custom_set_2.map)

    %__MODULE__{map: map}
  end
end
