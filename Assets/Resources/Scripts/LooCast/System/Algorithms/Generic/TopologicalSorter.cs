using System;
using System.Collections.Generic;
using System.Linq;

namespace LooCast.System.Algorithms.Generic
{
    public class TopologicalSorter<T> where T : notnull
    {
        private readonly IEqualityComparer<T> comparer;
        private readonly Func<T, IEnumerable<T>> dependencySelector;

        public TopologicalSorter(Func<T, IEnumerable<T>> dependencySelector, IEqualityComparer<T>? comparer = null)
        {
            this.comparer = comparer ?? EqualityComparer<T>.Default;
            this.dependencySelector = dependencySelector ?? throw new ArgumentNullException(nameof(dependencySelector));
        }

        private void EnsureNoDuplicates(IEnumerable<T> items, string argumentName)
        {
            var set = new HashSet<T>(comparer);
            foreach (var item in items)
            {
                if (!set.Add(item))
                {
                    throw new ArgumentException($"The collection contains duplicates: {item}", argumentName);
                }
            }
        }

        private void EnsureNoIntersections(IEnumerable<T> unresolvedItems, IEnumerable<T> resolvedItems)
        {
            var resolvedSet = new HashSet<T>(resolvedItems, comparer);
            foreach (var item in unresolvedItems)
            {
                if (resolvedSet.Contains(item))
                {
                    throw new ArgumentException($"The resolvedItems collection contains unresolvedItems that are already in the unresolvedItems collection: {item}", nameof(resolvedItems));
                }
            }
        }

        private Dictionary<T, HashSet<T>> BuildDependenciesMap(IEnumerable<T> combinedItems)
        {
            var dependenciesMap = new Dictionary<T, HashSet<T>>();
            foreach (var item in combinedItems)
            {
                EnsureNoDuplicates(dependencySelector(item), nameof(combinedItems));

                var missingDependencies = dependencySelector(item).Where(dep => !combinedItems.Contains(dep));
                if (missingDependencies.Any())
                {
                    throw new ArgumentException($"Not all dependencies are included in the input collections. Missing dependencies: {string.Join(", ", missingDependencies)}", nameof(combinedItems));
                }

                dependenciesMap[item] = new HashSet<T>(dependencySelector(item), comparer);
            }
            return dependenciesMap;
        }

        private void DetectCircularDependency(T current, HashSet<T> path, Dictionary<T, HashSet<T>> dependenciesMap)
        {
            if (!path.Add(current))
            {
                throw new Exception($"Circular dependency detected: {string.Join(" -> ", path)}");
            }

            foreach (T next in dependenciesMap[current])
            {
                DetectCircularDependency(next, path, dependenciesMap);
            }
            path.Remove(current);
        }

        private void EnsureNoCircularDependencies(Dictionary<T, HashSet<T>> dependenciesMap)
        {
            var visitedNodes = new HashSet<T>(comparer);
            foreach (var item in dependenciesMap.Keys)
            {
                if (!visitedNodes.Contains(item))
                {
                    DetectCircularDependency(item, new HashSet<T>(comparer), dependenciesMap);
                }
            }
        }

        private List<HashSet<T>> SortItems(Dictionary<T, HashSet<T>> dependenciesMap)
        {
            var sortedGroups = new List<HashSet<T>>();
            var itemsWithoutDependencies = new Queue<T>(dependenciesMap.Keys.Where(i => !dependenciesMap[i].Any()));

            while (itemsWithoutDependencies.Count > 0)
            {
                var currentGroup = new HashSet<T>(comparer);
                int itemsCount = itemsWithoutDependencies.Count;

                for (int i = 0; i < itemsCount; i++)
                {
                    var currentItem = itemsWithoutDependencies.Dequeue();
                    currentGroup.Add(currentItem);

                    foreach (var dependentItem in dependenciesMap.Keys.ToList())
                    {
                        if (dependenciesMap[dependentItem].Remove(currentItem) && !dependenciesMap[dependentItem].Any())
                        {
                            itemsWithoutDependencies.Enqueue(dependentItem);
                        }
                    }
                    dependenciesMap.Remove(currentItem);
                }
                sortedGroups.Add(currentGroup);
            }

            if (dependenciesMap.Count != 0)
            {
                throw new Exception($"Unidentifiable circular dependency detected! Remaining dependencies: {string.Join(", ", dependenciesMap.Select(kv => $"{kv.Key} -> {string.Join(", ", kv.Value)}"))}");
            }

            return sortedGroups;
        }

        public IEnumerable<IEnumerable<T>> Sort(IEnumerable<T> unresolvedItems, IEnumerable<T>? resolvedItems = null)
        {
            EnsureNoDuplicates(unresolvedItems, nameof(unresolvedItems));

            if (resolvedItems != null)
            {
                EnsureNoDuplicates(resolvedItems, nameof(resolvedItems));
                EnsureNoIntersections(unresolvedItems, resolvedItems);
            }

            var combinedItems = new HashSet<T>(unresolvedItems, comparer);
            if (resolvedItems != null)
            {
                combinedItems.UnionWith(resolvedItems);
            }

            var dependenciesMap = BuildDependenciesMap(combinedItems);
            EnsureNoCircularDependencies(dependenciesMap);

            if (resolvedItems != null)
            {
                foreach (var unresolvedItem in unresolvedItems)
                {
                    dependenciesMap[unresolvedItem].ExceptWith(resolvedItems);
                }
                foreach (var resolvedItem in resolvedItems)
                {
                    dependenciesMap.Remove(resolvedItem);
                }
            }

            return SortItems(dependenciesMap);
        }
    }
}