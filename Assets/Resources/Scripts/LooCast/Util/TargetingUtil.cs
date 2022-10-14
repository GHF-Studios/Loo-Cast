using System;
using System.Linq;
using UnityEngine;

public static class TargetingUtil
{
    public enum SortingType
    {
        Closest,
        Furthest
    }

    #region Get Single Target
    public static Collider2D GetTarget(Vector2 samplePosition, float sampleRadius)
    {
        return Physics2D.OverlapCircle(samplePosition, sampleRadius);
    }

    public static Collider2D GetTarget(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
    {
        return Physics2D.OverlapCircle(samplePosition, sampleRadius, layerMask);
    }

    public static Collider2D GetTarget(Bounds sampleBounds)
    {
        return Physics2D.OverlapArea(sampleBounds.min, sampleBounds.max);
    }

    public static Collider2D GetTarget(Bounds sampleBounds, LayerMask layerMask)
    {
        return Physics2D.OverlapArea(sampleBounds.min, sampleBounds.max, layerMask);
    }

    public static Collider2D GetTarget(Vector2 samplePoint)
    {
        return Physics2D.OverlapPoint(samplePoint);
    }

    public static Collider2D GetTarget(Vector2 samplePoint, LayerMask layerMask)
    {
        return Physics2D.OverlapPoint(samplePoint, layerMask);
    }
    #endregion

    #region Get Multiple Targets
    public static Collider2D[] GetTargets(Vector2 samplePosition, float sampleRadius)
    {
        return Physics2D.OverlapCircleAll(samplePosition, sampleRadius);
    }

    public static Collider2D[] GetTargets(Vector2 samplePosition, float sampleRadius, LayerMask layerMask)
    {
        return Physics2D.OverlapCircleAll(samplePosition, sampleRadius, layerMask);
    }

    public static Collider2D[] GetTargets(Bounds sampleBounds)
    {
        return Physics2D.OverlapAreaAll(sampleBounds.min, sampleBounds.max);
    }

    public static Collider2D[] GetTargets(Bounds sampleBounds, LayerMask layerMask)
    {
        return Physics2D.OverlapAreaAll(sampleBounds.min, sampleBounds.max, layerMask);
    }
    
    public static Collider2D[] GetTargets(Vector2 samplePoint)
    {
        return Physics2D.OverlapPointAll(samplePoint);
    }

    public static Collider2D[] GetTargets(Vector2 samplePoint, LayerMask layerMask)
    {
        return Physics2D.OverlapPointAll(samplePoint, layerMask);
    }
    #endregion

    #region Filter & Sort Targets
    public static Collider2D[] FilterTargets(Collider2D[] targets, string filterTag)
    {
        return targets.Where((target) => target.gameObject.CompareTag(filterTag)).ToArray();
    }

    public static Collider2D[] FilterTargets(Collider2D[] targets, string[] filterTags)
    {
        return targets.Where((target) => 
        {
            foreach (string filterTag in filterTags)
            {
                if (target.gameObject.CompareTag(filterTag))
                {
                    return true;
                }
            }
            return false;
        }).ToArray();
    }

    public static Collider2D[] FilterTargets(Collider2D[] targets, Type filterComponentType)
    {
        return targets.Where((target) =>
        {
            return target.TryGetComponent(filterComponentType, out _);
        }).ToArray();
    }

    public static Collider2D[] FilterTargets(Collider2D[] targets, Type[] filterComponentTypes)
    {
        return targets.Where((target) =>
        {
            foreach (Type filterComponentType in filterComponentTypes)
            {
                if (target.TryGetComponent(filterComponentType, out _))
                {
                    return true;
                }
            }
            return false;
        }).ToArray();
    }

    public static Collider2D[] FilterTargets(Collider2D[] targets, Collider2D ignoredTarget)
    {
        return targets.Where(target => !target.Equals(ignoredTarget)).ToArray();
    }

    public static Collider2D[] FilterTargets(Collider2D[] targets, Collider2D[] ignoredTargets)
    {
        return targets.Where((target) =>
        {
            foreach (Collider2D ignoredTarget in ignoredTargets)
            {
                if (target.Equals(ignoredTarget))
                {
                    return false;
                }
            }
            return true;
        }).ToArray();
    }

    public static Collider2D[] SortTargets(Collider2D[] targets, Vector2 measurementOrigin, SortingType sortingType)
    {
        switch (sortingType)
        {
            case SortingType.Closest:
                return targets.OrderBy(target => Vector2.Distance(measurementOrigin, target.gameObject.transform.position)).ToArray();
            case SortingType.Furthest:
                return targets.OrderByDescending(target => Vector2.Distance(measurementOrigin, target.gameObject.transform.position)).ToArray();
            default:
                return targets;
        }
    }
    #endregion

    #region Private Utility Methods
    private static bool ValidateTargets(Collider2D[] targets)
    {
        if (targets == null || targets.Length == 0)
        {
            return false;
        }
        return true;
    }
    #endregion
}
