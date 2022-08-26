using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Targeting
{
    using Data;
    using LooCast.Target;
    using LooCast.Data;

    public class Targeting : MonoBehaviour, ITargeting
    {
        #region Data
        [SerializeField] private TargetingData data;
        #endregion

        #region Properties
        public float radius;
        public string[] targetTags { get; protected set; }
        public bool drawGizmos { get; protected set; }

        public List<Target> ClosestTargets
        {
            get
            {
                return FilterTargets(GetClosestTargets(), IgnoredTargets);
            }

            protected set
            {
                closestTargets = value;
            }
        }
        public List<Target> FurthestTargets
        {
            get
            {
                return FilterTargets(GetFurthestTargets(), IgnoredTargets);
            }

            protected set
            {
                furthestTargets = value;
            }
        }
        public List<Target> RandomTargets
        {
            get
            {
                return FilterTargets(GetRandomTargets(), IgnoredTargets);
            }

            protected set
            {
                randomTargets = value;
            }
        }
        public List<Target> RandomOnscreenTargets
        {
            get
            {
                return FilterTargets(GetRandomOnscreenTargets(), IgnoredTargets);
            }

            protected set
            {
                randomOnscreenTargets = value;
            }
        }
        public List<Target> RandomProximityTargets
        {
            get
            {
                return FilterTargets(GetRandomProximityTargets(), IgnoredTargets);
            }

            protected set
            {
                randomProximityTargets = value;
            }
        }
        #endregion

        #region Fields
        public List<Target> IgnoredTargets;

        private List<Target> closestTargets;
        private List<Target> furthestTargets;
        private List<Target> randomTargets;
        private List<Target> randomOnscreenTargets;
        private List<Target> randomProximityTargets;
        private System.Random random;
        #endregion

        #region Unity Callbacks
        private void OnDrawGizmos()
        {
            if (drawGizmos)
            {
                Gizmos.DrawWireSphere(transform.position, radius);
            }
        }
        private void LateUpdate()
        {
            closestTargets = null;
            furthestTargets = null;
            randomTargets = null;
            randomOnscreenTargets = null;
        }
        #endregion

        #region Methods
        public void Initialize(TargetingData data)
        {
            radius = data.Radius.Value;
            targetTags = StringDataReference.Evaluate(data.TargetedTags);
            drawGizmos = data.DrawGizmos.Value;
            random = new System.Random(Mathf.RoundToInt(Time.time));
            IgnoredTargets = new List<Target>();
        }
        
        private bool CheckTags(Collider2D collider, params string[] tags)
        {
            foreach (string tag in tags)
            {
                if (collider.gameObject.CompareTag(tag))
                {
                    return true;
                }
            }
            return false;
        }
        private List<Target> FilterTargets(List<Target> targets, List<Target> ignoredTargets)
        {
            if (targets == null || targets.Count == 0)
            {
                return null;
            }
            if (ignoredTargets == null || ignoredTargets.Count == 0)
            {
                return targets;
            }

            foreach (Target ignoredTarget in ignoredTargets)
            {
                targets.RemoveAll(target => target.Equals(ignoredTarget));
            }

            if (targets.Count == 0)
            {
                return null;
            }
            return targets;
        }
        private List<Target> ValidateTargets(List<Target> targets)
        {
            targets.RemoveAll((target) => !target.IsValid() || target.IsLocked());
            if (targets == null || targets.Count == 0)
            {
                return null;
            }
            return targets;
        }
        private List<Target> GetClosestTargets()
        {
            if (closestTargets == null || closestTargets.Count == 0)
            {
                List<Collider2D> collisions = Physics2D.OverlapCircleAll(transform.position, radius).ToList();

                if (collisions == null || collisions.Count == 0)
                {
                    return null;
                }

                collisions.RemoveAll(collision => !CheckTags(collision, targetTags));

                if (collisions.Count <= 0)
                {
                    return null;
                }

                collisions = collisions.OrderBy(x => Vector2.Distance(transform.position, x.transform.position)).ToList();

                List<Target> targets = new List<Target>();
                foreach (Collider2D collision in collisions)
                {
                    targets.Add(new Target(collision));
                }

                targets = ValidateTargets(targets);

                closestTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(closestTargets);
            }
        }
        private List<Target> GetFurthestTargets()
        {
            if (furthestTargets == null || furthestTargets.Count == 0)
            {
                List<Collider2D> collisions = Physics2D.OverlapCircleAll(transform.position, radius).Reverse().ToList();

                if (collisions == null || collisions.Count == 0)
                {
                    return null;
                }

                collisions.RemoveAll(collision => !CheckTags(collision, targetTags));

                if (collisions.Count == 0)
                {
                    return null;
                }

                collisions = collisions.OrderBy(x => Vector2.Distance(transform.position, x.transform.position)).ToList();

                List<Target> targets = new List<Target>();
                foreach (Collider2D collision in collisions)
                {
                    targets.Add(new Target(collision));
                }

                targets = ValidateTargets(targets);

                furthestTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(furthestTargets);
            }
        }
        private List<Target> GetRandomTargets()
        {
            if (randomTargets == null || randomTargets.Count == 0)
            {
                List<Target> targets = new List<Target>();
                foreach (string targetTag in targetTags)
                {
                    targets.AddRange(GameObject.FindGameObjectsWithTag(targetTag).ToList().ConvertAll(x => new Target(x.GetComponent<Collider2D>())));
                }
                if (targets == null || targets.Count == 0)
                {
                    return null;
                }
                targets = targets.OrderBy(x => random.Next()).ToList();
                targets = ValidateTargets(targets);

                randomTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(randomTargets);
            }
        }
        private List<Target> GetRandomOnscreenTargets()
        {
            if (randomOnscreenTargets == null || randomOnscreenTargets.Count == 0)
            {
                List<Collider2D> collisions = Physics2D.OverlapAreaAll(Camera.main.ScreenToWorldPoint(Vector3.zero), Camera.main.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height))).ToList();

                if (collisions == null || collisions.Count <= 0)
                {
                    return null;
                }

                collisions.RemoveAll(collision => !CheckTags(collision, targetTags));

                if (collisions.Count <= 0)
                {
                    return null;
                }

                List<Target> targets = new List<Target>();
                foreach (Collider2D collision in collisions)
                {
                    targets.Add(new Target(collision));
                }

                targets = ValidateTargets(targets);

                randomOnscreenTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(randomOnscreenTargets);
            }
        }
        private List<Target> GetRandomProximityTargets()
        {
            if (randomProximityTargets == null || randomProximityTargets.Count == 0)
            {
                List<Target> targets = ClosestTargets;
                if (targets == null || targets.Count == 0)
                {
                    return null;
                }
                targets = targets.OrderBy(x => random.Next()).ToList();
                targets = ValidateTargets(targets);

                randomProximityTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(randomProximityTargets);
            }
        }
        #endregion
    }
}
