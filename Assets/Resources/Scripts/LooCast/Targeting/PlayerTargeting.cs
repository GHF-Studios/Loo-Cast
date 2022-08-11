using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Targeting
{
    using Data;
    using Data.Runtime;
    using LooCast.Variable;
    using LooCast.Target;

    [DisallowMultipleComponent]
    public class PlayerTargeting : MonoBehaviour, ITargeting
    {
        public PlayerTargetingData Data;
        public PlayerTargetingRuntimeData RuntimeData;

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

        public List<Target> closestTargets
        {
            get
            {
                return FilterTargets(GetClosestTargets());
            }

            protected set
            {
                _closestTargets = value;
            }
        }
        private List<Target> _closestTargets;

        public List<Target> furthestTargets
        {
            get
            {
                return FilterTargets(GetFurthestTargets());
            }

            protected set
            {
                _furthestTargets = value;
            }
        }
        private List<Target> _furthestTargets;

        public List<Target> randomTargets
        {
            get
            {
                return FilterTargets(GetRandomTargets());
            }

            protected set
            {
                _randomTargets = value;
            }
        }
        private List<Target> _randomTargets;

        public List<Target> randomOnscreenTargets
        {
            get
            {
                return FilterTargets(GetRandomOnscreenTargets());
            }

            protected set
            {
                _randomOnscreenTargets = value;
            }
        }
        private List<Target> _randomOnscreenTargets;

        public List<Target> randomProximityTargets
        {
            get
            {
                return FilterTargets(GetRandomProximityTargets());
            }

            protected set
            {
                _randomProximityTargets = value;
            }
        }
        private List<Target> _randomProximityTargets;

        private void OnDrawGizmos()
        {
            if (RuntimeData.DrawGizmos.Value)
            {
                Gizmos.DrawWireSphere(transform.position, RuntimeData.Radius.Value);
            }
        }

        private void LateUpdate()
        {
            _closestTargets = null;
            _furthestTargets = null;
            _randomTargets = null;
            _randomOnscreenTargets = null;
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
            if (_closestTargets == null || _closestTargets.Count == 0)
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

                _closestTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(_closestTargets);
            }
        }

        private List<Target> GetFurthestTargets()
        {
            if (_furthestTargets == null || _furthestTargets.Count == 0)
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

                _furthestTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(_furthestTargets);
            }
        }

        private List<Target> GetRandomTargets()
        {
            if (_randomTargets == null || _randomTargets.Count == 0)
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

                _randomTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(_randomTargets);
            }
        }

        private List<Target> GetRandomOnscreenTargets()
        {
            if (_randomOnscreenTargets == null || _randomOnscreenTargets.Count == 0)
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

                _randomOnscreenTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(_randomOnscreenTargets);
            }
        }

        private List<Target> GetRandomProximityTargets()
        {
            if (_randomProximityTargets == null || _randomProximityTargets.Count == 0)
            {
                List<Target> targets = closestTargets;
                if (targets == null || targets.Count == 0)
                {
                    return null;
                }
                targets = targets.OrderBy(x => RuntimeData.Random.Next()).ToList();
                targets = ValidateTargets(targets);

                _randomProximityTargets = targets;
                return targets;
            }
            else
            {
                return ValidateTargets(_randomProximityTargets);
            }
        }
    }
}
