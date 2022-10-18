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
        public static NewTarget GetTarget(Vector2 samplePosition, float sampleRadius)
        {
            return Physics2D.OverlapCircle(samplePosition, sampleRadius).GetTarget();
        }

        public static NewTarget GetTarget(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            return Physics2D.OverlapCircle(samplePosition, sampleRadius, layerMask).GetTarget();
        }

        public static NewTarget GetTarget(Bounds sampleBounds)
        {
            return Physics2D.OverlapArea(sampleBounds.min, sampleBounds.max).GetTarget();
        }

        public static NewTarget GetTarget(Bounds sampleBounds, LayerMask layerMask)
        {
            return Physics2D.OverlapArea(sampleBounds.min, sampleBounds.max, layerMask).GetTarget();
        }

        public static NewTarget GetTarget(Vector2 samplePoint)
        {
            return Physics2D.OverlapPoint(samplePoint).GetTarget();
        }

        public static NewTarget GetTarget(Vector2 samplePoint, LayerMask layerMask)
        {
            return Physics2D.OverlapPoint(samplePoint, layerMask).GetTarget();
        }
        #endregion

        #region Get Multiple Targets
        public static NewTarget[] GetTargets(Vector2 samplePosition, float sampleRadius)
        {
            return Physics2D.OverlapCircleAll(samplePosition, sampleRadius).GetTargets();
        }

        public static NewTarget[] GetTargets(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
        {
            return Physics2D.OverlapCircleAll(samplePosition, sampleRadius, layerMask).GetTargets();
        }

        public static NewTarget[] GetTargets(Bounds sampleBounds)
        {
            return Physics2D.OverlapAreaAll(sampleBounds.min, sampleBounds.max).GetTargets();
        }

        public static NewTarget[] GetTargets(Bounds sampleBounds, LayerMask layerMask)
        {
            return Physics2D.OverlapAreaAll(sampleBounds.min, sampleBounds.max, layerMask).GetTargets();
        }

        public static NewTarget[] GetTargets(Vector2 samplePoint)
        {
            return Physics2D.OverlapPointAll(samplePoint).GetTargets();
        }

        public static NewTarget[] GetTargets(Vector2 samplePoint, LayerMask layerMask)
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
