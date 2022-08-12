using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Targeting
{
    using Data;
    using LooCast.Target;

    public class PlayerStationTargeting : MonoBehaviour, ITargeting
    {
        public PlayerStationTargetingData Data;

        public float radius;
        public string[] targetTags;
        public bool drawGizmos;
        private System.Random random;
        public List<Target> ignoredTargets;

        public List<Target> closestTargets
        {
            get
            {
                return FilterTargets(GetClosestTargets(), ignoredTargets);
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
                return FilterTargets(GetFurthestTargets(), ignoredTargets);
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
                return FilterTargets(GetRandomTargets(), ignoredTargets);
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
                return FilterTargets(GetRandomOnscreenTargets(), ignoredTargets);
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
                return FilterTargets(GetRandomProximityTargets(), ignoredTargets);
            }

            protected set
            {
                _randomProximityTargets = value;
            }
        }
        private List<Target> _randomProximityTargets;

        private void Start()
        {
            radius = Data.Radius.Value;
            targetTags = new string[Data.TargetedTags.Length];
            for (int i = 0; i < targetTags.Length; i++)
            {
                targetTags[i] = Data.TargetedTags[i].Value;
            }
            drawGizmos = Data.DrawGizmos.Value;
            random = new System.Random(Mathf.RoundToInt(Time.time));
            ignoredTargets = new List<Target>();
        }

        private void OnDrawGizmos()
        {
            if (drawGizmos)
            {
                Gizmos.DrawWireSphere(transform.position, radius);
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
            if (_closestTargets == null || _closestTargets.Count == 0)
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
                targets = targets.OrderBy(x => random.Next()).ToList();
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
