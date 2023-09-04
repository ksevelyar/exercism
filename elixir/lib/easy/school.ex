defmodule School do
  @moduledoc """
  Simulate students in a school.

  Each student is in a grade.
  """
  defstruct roster: %{}

  @type school :: any()

  @doc """
  Create a new, empty school.
  """
  @spec new() :: school
  def new(), do: %School{}

  @doc """
  Add a student to a particular grade in school.
  """
  @spec add(school, String.t(), integer) :: {:ok | :error, school}
  def add(school, name, grade) do
    if school.roster[name] do
      {:error, school}
    else
      roster = Map.put(school.roster, name, grade)
      {:ok, %{school | roster: roster}}
    end
  end

  @doc """
  Return the names of the students in a particular grade, sorted alphabetically.
  """
  @spec grade(school, integer) :: [String.t()]
  def grade(school, grade) do
    school.roster |> Enum.filter(&(elem(&1, 1) == grade)) |> Enum.map(&elem(&1, 0)) |> Enum.sort()
  end

  @doc """
  Return the names of all the students in the school sorted by grade and name.
  """
  @spec roster(school) :: [String.t()]
  def roster(school) do
    school.roster |> Enum.sort_by(&{elem(&1, 1), elem(&1, 0)}) |> Enum.map(&elem(&1, 0))
  end
end
