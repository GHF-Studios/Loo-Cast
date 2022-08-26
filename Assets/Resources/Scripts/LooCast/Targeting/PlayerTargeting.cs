using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Targeting
{
    using Data;
    using Data.Runtime;
    using LooCast.Variable;
    using LooCast.Target;

    public sealed class PlayerTargeting : MonoBehaviour, ITargeting
    {
        #region Data
        [SerializeField] private PlayerTargetingData Data;
        [SerializeField] private PlayerTargetingRuntimeData RuntimeData;
        #endregion

        #region Properties
        public List<Target> ClosestTargets
        {
            get
            {
                return FilterTargets(GetClosestTargets());
            }

            private set
            {
                closestTargets = value;
            }
        }
        public List<Target> FurthestTargets
        {
            get
            {
                return FilterTargets(GetFurthestTargets());
            }

            private set
            {
                furthestTargets = value;
            }
        }
        public List<Target> RandomTargets
        {
            get
            {
                return FilterTargets(GetRandomTargets());
            }

            private set
            {
                randomTargets = value;
            }
        }
        public List<Target> RandomOnscreenTargets
        {
            get
            {
                return FilterTargets(GetRandomOnscreenTargets());
            }

            private set
            {
                randomOnscreenTargets = value;
            }
        }
        public List<Target> RandomProximityTargets
        {
            get
            {
                return FilterTargets(GetRandomProximityTargets());
            }

            private set
            {
                randomProximityTargets = value;
            }
        }
        #endregion

        #region Fields
        private List<Target> closestTargets;
        private List<Target> furthestTargets;
        private List<Target> randomTargets;
        private List<Target> randomOnscreenTargets;
        private List<Target> randomProximityTargets;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            RuntimeData.Radius = new FloatComputedVariable(Data.Radius.Value);
            RuntimeData.TargetTags = new StringVariable[Data.TargetedTags.Length];
            for (int i = 0; i < RuntimeData.TargetTags.Length; i++)
            {
                RuntimeData.TargetTags[i] = new StringVariable(Data.TargetedTags[i].Value);
            }
            RuntimeData.DrawGizmos = new BoolVariable(Data.DrawGizmos.Value);
            RuntimeData.Random = new System.Random(Mathf.RoundToInt(Time.time));
            RuntimeData.IgnoredTargets = new List<Target>();
        }
        private void OnDrawGizmos()
        {
            if (RuntimeData.DrawGizmos.Value)
            {
                Gizmos.DrawWireSphere(transform.position, RuntimeData.Radius.Value);
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
        private List<Target> FilterTargets(List<Target> targets)
        {
            if (targets == null || targets.Count == 0)
            {
                return null;
            }
            if (RuntimeData.IgnoredTargets == null || RuntimeData.IgnoredTargets.Count == 0)
            {
                return targets;
            }

            foreach (Target ignoredTarget in RuntimeData.IgnoredTargets)
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
                List<Collider2D> collisions = Physics2D.OverlapCircleAll(transform.position, RuntimeData.Radius.Value).ToList();

                if (collisions == null || collisions.Count == 0)
                {
                    return null;
                }

                collisions.RemoveAll(collision => !CheckTags(collision, Variable<string>.Evaluate(RuntimeData.TargetTags)));

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
                List<Collider2D> collisions = Physics2D.OverlapCircleAll(transform.position, RuntimeData.Radius.Value).Reverse().ToList();

                if (collisions == null || collisions.Count == 0)
                {
                    return null;
                }

                collisions.RemoveAll(collision => !CheckTags(collision, Variable<string>.Evaluate(RuntimeData.TargetTags)));

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
                foreach (string targetTag in Variable<string>.Evaluate(RuntimeData.TargetTags))
                {
                    targets.AddRange(GameObject.FindGameObjectsWithTag(targetTag).ToList().ConvertAll(x => new Target(x.GetComponent<Collider2D>())));
                }
                if (targets == null || targets.Count == 0)
                {
                    return null;
                }
                targets = targets.OrderBy(x => RuntimeData.Random.Next()).ToList();
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

                collisions.RemoveAll(collision => !CheckTags(collision, Variable<string>.Evaluate(RuntimeData.TargetTags)));

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
                targets = targets.OrderBy(x => RuntimeData.Random.Next()).ToList();
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
