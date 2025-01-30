defmodule Pov do
  @typedoc """
  A tree, which is made of a node with several branches.
  """
  @type tree :: {any, [tree]}

  @doc """
  Reroot a tree on a selected node.
  """
  @spec from_pov(tree, any) :: {:ok, tree} | {:error, atom}
  def from_pov(tree, node) do
    reroot_tree(tree, node, [])
  end

  defp reroot_tree({target, children}, target, acc) do
    {:ok, {target, children ++ acc}}
  end

  defp reroot_tree({_value, []}, _target, _acc) do
    {:error, :nonexistent_target}
  end

  defp reroot_tree({value, children}, target, acc) do
    Enum.find_value(children, {:error, :nonexistent_target}, fn child ->
      new_path = [{value, acc ++ (children -- [child])}]

      case reroot_tree(child, target, new_path) do
        {:ok, tree} -> {:ok, tree}
        {:error, :nonexistent_target} -> nil
      end
    end)
  end

  @doc """
  Finds a path between two nodes.
  """
  @spec path_between(tree, any, any) :: {:ok, [any]} | {:error, atom}
  def path_between(tree, from, to) do
    case reroot_tree(tree, from, []) do
      {:ok, new_tree} -> find_path(new_tree, to, [])
      {:error, _} -> {:error, :nonexistent_source}
    end
  end

  defp find_path({target, _children}, target, acc) do
    {:ok, acc ++ [target]}
  end

  defp find_path({_value, []}, _target, _acc) do
    {:error, :nonexistent_destination}
  end

  defp find_path({value, children}, target, acc) do
    Enum.find_value(children, {:error, :nonexistent_destination}, fn child ->
      case find_path(child, target, acc ++ [value]) do
        {:ok, path} -> {:ok, path}
        {:error, :nonexistent_destination} -> nil
      end
    end)
  end
end
