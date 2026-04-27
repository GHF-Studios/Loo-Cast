using System;
using System.Linq;
using UnityEngine;

namespace LooCast.Util
{
    using LooCast.Health;
    using Target;
    using UnityEngine.UIElements;

    public static class TargetingUtil
    {
        public enum SortingType
        {
            Closest,
            Furthest
        }

        #region Get Single Target
        public static Target GetTargetInRadius(Vector2 samplePosition, float sampleRadius)
        {
            return Physics2D.OverlapCircle(samplePosition, sampleRadius).GetTarget();
        }

        public static Target GetTargetInRadius(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            return Physics2D.OverlapCircle(samplePosition, sampleRadius, layerMask).GetTarget();
        }

        public static Target GetTargetInArea(Vector2 samplePositionA, Vector2 samplePositionB)
        {
            return Physics2D.OverlapArea(samplePositionA, samplePositionB).GetTarget();
        }

        public static Target GetTargetInArea(Vector2 samplePositionA, Vector2 samplePositionB, LayerMask layerMask)
        {
            return Physics2D.OverlapArea(samplePositionA, samplePositionB, layerMask).GetTarget();
        }

        public static Target GetTargetInPoint(Vector2 samplePoint)
        {
            return Physics2D.OverlapPoint(samplePoint).GetTarget();
        }

        public static Target GetTargetInPoint(Vector2 samplePoint, LayerMask layerMask)
        {
            return Physics2D.OverlapPoint(samplePoint, layerMask).GetTarget();
        }
        #endregion

        #region Get Multiple Targets
        public static Target[] GetTargetsInRadius(Vector2 samplePosition, float sampleRadius)
        {
            return Physics2D.OverlapCircleAll(samplePosition, sampleRadius).GetTargets();
        }

        public static Target[] GetTargetsInRadius(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            return Physics2D.OverlapCircleAll(samplePosition, sampleRadius, layerMask).GetTargets();
        }

        public static Target[] GetTargetsInArea(Vector2 samplePositionA, Vector2 samplePositionB)
        {
            return Physics2D.OverlapAreaAll(samplePositionA, samplePositionB).GetTargets();
        }

        public static Target[] GetTargetsInArea(Vector2 samplePositionA, Vector2 samplePositionB, LayerMask layerMask)
        {
            return Physics2D.OverlapAreaAll(samplePositionA, samplePositionB, layerMask).GetTargets();
        }

        public static Target[] GetTargetsInPoint(Vector2 samplePoint)
        {
            return Physics2D.OverlapPointAll(samplePoint).GetTargets();
        }

        public static Target[] GetTargetsInPoint(Vector2 samplePoint, LayerMask layerMask)
        {
            return Physics2D.OverlapPointAll(samplePoint, layerMask).GetTargets();
        }
        #endregion

        #region Filter & Sort Targets
        public static Target[] FilterTargets(Target[] targets, string filterTag)
        {
            return targets.Where((target) => target.GameObject.CompareTag(filterTag)).ToArray();
        }
        public static Target[] FilterTargets(Target[] targets, string[] filterTags)
        {
            return targets.Where((target) =>
            {
                foreach (string filterTag in filterTags)
                {
                    if (target.GameObject.CompareTag(filterTag))
                    {
                        return true;
                    }
                }
                return false;
            }).ToArray();
        }
        public static Target[] FilterTargets(Target[] targets, Type filterComponentType)
        {
            return targets.Where((target) =>
            {
                return target.GameObject.TryGetComponent(filterComponentType, out _);
            }).ToArray();
        }
        public static Target[] FilterTargets(Target[] targets, Type[] filterComponentTypes)
        {
            return targets.Where((target) =>
            {
                foreach (Type filterComponentType in filterComponentTypes)
                {
                    if (target.GameObject.TryGetComponent(filterComponentType, out _))
                    {
                        return true;
                    }
                }
                return false;
            }).ToArray();
        }
        public static Target[] FilterTargets(Target[] targets, Target ignoredTarget)
        {
            return targets.Where(target => !target.Equals(ignoredTarget)).ToArray();
        }
        public static Target[] FilterTargets(Target[] targets, Target[] ignoredTargets)
        {
            return targets.Where((target) =>
            {
                foreach (Target ignoredTarget in ignoredTargets)
                {
                    if (target.Equals(ignoredTarget))
                    {
                        return false;
                    }
                }
                return true;
            }).ToArray();
        }

        public static Target[] SortTargets(Target[] targets, Vector2 measurementOrigin, SortingType sortingType)
        {
            switch (sortingType)
            {
                case SortingType.Closest:
                    return targets.OrderBy(target => Vector2.Distance(measurementOrigin, target.Transform.position)).ToArray();
                case SortingType.Furthest:
                    return targets.OrderByDescending(target => Vector2.Distance(measurementOrigin, target.Transform.position)).ToArray();
                default:
                    return targets;
            }
        }
        #endregion

        #region Common Use Case Abstractions
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterTags);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, Target[] ignoredTargets)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, ignoredTargets);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, Type[] filterComponentTypes)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterComponentTypes);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, filterTags);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, Target[] ignoredTargets, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, ignoredTargets);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }
        public static Target[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, Type[] filterComponentTypes, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, filterComponentTypes);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }

        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }
        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }
        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterTags);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }
        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, Target[] ignoredTargets)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, ignoredTargets);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }
        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, Type[] filterComponentTypes)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterComponentTypes);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }
        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, filterTags);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }
        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, Target[] ignoredTargets, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, ignoredTargets);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }
        public static Target[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, Type[] filterComponentTypes, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, filterComponentTypes);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }

        public static Target[] GetRandomOnscreenTargets()
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB);
            return targets;
        }
        public static Target[] GetRandomOnscreenTargets(LayerMask layerMask)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB, layerMask);
            return targets;
        }
        public static Target[] GetRandomOnscreenTargets(string[] filterTags)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB);
            targets = FilterTargets(targets, filterTags);
            return targets;
        }
        public static Target[] GetRandomOnscreenTargets(Target[] ignoredTargets)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB);
            targets = FilterTargets(targets, ignoredTargets);
            return targets;
        }
        public static Target[] GetRandomOnscreenTargets(Type[] filterComponentTypes)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB);
            targets = FilterTargets(targets, filterComponentTypes);
            return targets;
        }
        public static Target[] GetRandomOnscreenTargets(string[] filterTags, LayerMask layerMask)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB, layerMask);
            targets = FilterTargets(targets, filterTags);
            return targets;
        }
        public static Target[] GetRandomOnscreenTargets(Target[] ignoredTargets, LayerMask layerMask)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB, layerMask);
            targets = FilterTargets(targets, ignoredTargets);
            return targets;
        }
        public static Target[] GetRandomOnscreenTargets(Type[] filterComponentTypes, LayerMask layerMask)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            Target[] targets = GetTargetsInArea(samplePositionA, samplePositionB, layerMask);
            targets = FilterTargets(targets, filterComponentTypes);
            return targets;
        }

        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            return targets;
        }
        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            return targets;
        }
        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterTags);
            return targets;
        }
        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, Target[] ignoredTargets)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, ignoredTargets);
            return targets;
        }
        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, Type[] filterComponentTypes)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterComponentTypes);
            return targets;
        }
        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, filterTags);
            return targets;
        }
        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, Target[] ignoredTargets, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, ignoredTargets);
            return targets;
        }
        public static Target[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, Type[] filterComponentTypes, LayerMask layerMask)
        {
            Target[] targets = GetTargetsInRadius(samplePosition, sampleRadius, layerMask);
            targets = FilterTargets(targets, filterComponentTypes);
            return targets;
        }
        #endregion

        #region Private Utility Methods
        private static Target GetTarget(this Collider2D targetCollider)
        {
            if (targetCollider == null)
            {
                return null;
            }
            IHealth targetHealth = targetCollider.gameObject.GetComponentInParent<IHealth>();
            if (targetHealth == null)
            {
                throw new Exception("Target must contain an IHealth Component!");
            }
            return new Target(targetHealth, targetCollider);
        }

        private static Target[] GetTargets(this Collider2D[] targetColliders)
        {
            return targetColliders.Select(targetCollider => targetCollider.GetTarget()).ToArray();
        }
        #endregion
    }
}
