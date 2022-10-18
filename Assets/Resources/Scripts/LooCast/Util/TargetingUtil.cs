using System;
using System.Linq;
using UnityEngine;

namespace LooCast.Util
{
    using LooCast.Health;
    using Target;

    public static class TargetingUtil
    {
        public enum SortingType
        {
            Closest,
            Furthest
        }

        #region Get Single Target
        public static NewTarget GetTargetInRadius(Vector2 samplePosition, float sampleRadius)
        {
            return Physics2D.OverlapCircle(samplePosition, sampleRadius).GetTarget();
        }

        public static NewTarget GetTargetInRadius(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            return Physics2D.OverlapCircle(samplePosition, sampleRadius, layerMask).GetTarget();
        }

        public static NewTarget GetTargetInArea(Vector2 samplePositionA, Vector2 samplePositionB)
        {
            return Physics2D.OverlapArea(samplePositionA, samplePositionB).GetTarget();
        }

        public static NewTarget GetTargetInArea(Vector2 samplePositionA, Vector2 samplePositionB, LayerMask layerMask)
        {
            return Physics2D.OverlapArea(samplePositionA, samplePositionB, layerMask).GetTarget();
        }

        public static NewTarget GetTargetInPoint(Vector2 samplePoint)
        {
            return Physics2D.OverlapPoint(samplePoint).GetTarget();
        }

        public static NewTarget GetTargetInPoint(Vector2 samplePoint, LayerMask layerMask)
        {
            return Physics2D.OverlapPoint(samplePoint, layerMask).GetTarget();
        }
        #endregion

        #region Get Multiple Targets
        public static NewTarget[] GetTargetsInRadius(Vector2 samplePosition, float sampleRadius)
        {
            return Physics2D.OverlapCircleAll(samplePosition, sampleRadius).GetTargets();
        }

        public static NewTarget[] GetTargetsInRadius(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            return Physics2D.OverlapCircleAll(samplePosition, sampleRadius, layerMask).GetTargets();
        }

        public static NewTarget[] GetTargetsInArea(Vector2 samplePositionA, Vector2 samplePositionB)
        {
            return Physics2D.OverlapAreaAll(samplePositionA, samplePositionB).GetTargets();
        }

        public static NewTarget[] GetTargetsInArea(Vector2 samplePositionA, Vector2 samplePositionB, LayerMask layerMask)
        {
            return Physics2D.OverlapAreaAll(samplePositionA, samplePositionB, layerMask).GetTargets();
        }

        public static NewTarget[] GetTargetsInPoint(Vector2 samplePoint)
        {
            return Physics2D.OverlapPointAll(samplePoint).GetTargets();
        }

        public static NewTarget[] GetTargetsInPoint(Vector2 samplePoint, LayerMask layerMask)
        {
            return Physics2D.OverlapPointAll(samplePoint, layerMask).GetTargets();
        }
        #endregion

        #region Filter & Sort Targets
        public static NewTarget[] FilterTargets(NewTarget[] targets, string filterTag)
        {
            return targets.Where((target) => target.GameObject.CompareTag(filterTag)).ToArray();
        }

        public static NewTarget[] FilterTargets(NewTarget[] targets, string[] filterTags)
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

        public static NewTarget[] FilterTargets(NewTarget[] targets, Type filterComponentType)
        {
            return targets.Where((target) =>
            {
                return target.GameObject.TryGetComponent(filterComponentType, out _);
            }).ToArray();
        }

        public static NewTarget[] FilterTargets(NewTarget[] targets, Type[] filterComponentTypes)
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

        public static NewTarget[] FilterTargets(NewTarget[] targets, NewTarget ignoredTarget)
        {
            return targets.Where(target => !target.Equals(ignoredTarget)).ToArray();
        }

        public static NewTarget[] FilterTargets(NewTarget[] targets, NewTarget[] ignoredTargets)
        {
            return targets.Where((target) =>
            {
                foreach (NewTarget ignoredTarget in ignoredTargets)
                {
                    if (target.Equals(ignoredTarget))
                    {
                        return false;
                    }
                }
                return true;
            }).ToArray();
        }

        public static NewTarget[] SortTargets(NewTarget[] targets, Vector2 measurementOrigin, SortingType sortingType)
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

        #region Common Use Cases
        public static NewTarget[] GetClosestTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags)
        {
            NewTarget[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterTags);
            targets = SortTargets(targets, samplePosition, SortingType.Closest);
            return targets;
        }

        public static NewTarget[] GetFurthestTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags)
        {
            NewTarget[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterTags);
            targets = SortTargets(targets, samplePosition, SortingType.Furthest);
            return targets;
        }

        public static NewTarget[] GetRandomOnscreenTargets(string[] filterTags)
        {
            Vector2 samplePositionA = Camera.main.ScreenToWorldPoint(new Vector3(0, 0));
            Vector2 samplePositionB = Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height));
            NewTarget[] targets = GetTargetsInArea(samplePositionA, samplePositionB);
            targets = FilterTargets(targets, filterTags);
            return targets;
        }

        public static NewTarget[] GetRandomProximityTargets(Vector3 samplePosition, float sampleRadius, string[] filterTags)
        {
            NewTarget[] targets = GetTargetsInRadius(samplePosition, sampleRadius);
            targets = FilterTargets(targets, filterTags);
            return targets;
        }
        #endregion

        #region Private Utility Methods
        private static NewTarget GetTarget(this Collider2D targetCollider)
        {
            IHealth targetHealth = targetCollider.gameObject.GetComponent<IHealth>();
            if (targetHealth == null)
            {
                throw new Exception("Target must contain an IHealth Component!");
            }
            return new NewTarget(targetHealth, targetCollider);
        }

        private static NewTarget[] GetTargets(this Collider2D[] targetColliders)
        {
            return targetColliders.Select(targetCollider => targetCollider.GetTarget()).ToArray();
        }
        #endregion
    }
}
