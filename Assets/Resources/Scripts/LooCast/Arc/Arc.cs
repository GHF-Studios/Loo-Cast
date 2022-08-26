using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Arc
{
    using Core;
    using Targeting;
    using Target;

    public class Arc : ExtendedMonoBehaviour
    {
        #region Fields
        [SerializeField] private Targeting targeting;
        public List<Target> targets;
        [HideInInspector] public List<ArcSegment> arcSegments;
        [HideInInspector] public ArcSegment arcSegment;
        private Arc nextArc;
        private Arc nextBranch;
        private List<Arc> arcs;




        private float lifetime;
        private float maxLifetime;

        [HideInInspector] public float width;
        private float widthMultiplier;
        [HideInInspector] public float minWidth;
        private int branchTries;



        private float minSpreadDistance;
        private float minSpreadDistanceMultiplier;

        private float maxSpreadDistance;
        private float maxSpreadDistanceMultiplier;

        private float minSpreadAngle;
        private float minSpreadAngleMultiplier;

        private float maxSpreadAngle;
        private float maxSpreadAngleMultiplier;

        private float spreadChance;
        private float spreadChanceMultiplier;



        private float minBranchDistance;
        private float minBranchDistanceMultiplier;

        private float maxBranchDistance;
        private float maxBranchDistanceMultiplier;

        private float minBranchAngle;
        private float minBranchAngleMultiplier;

        private float maxBranchAngle;
        private float maxBranchAngleMultiplier;

        private float branchChance;
        private float branchChanceMultiplier;

        public bool isMainArc { get; protected set; }

        public int recursion { get; protected set; }
        public int maxRecursion { get; protected set; }
        #endregion


        public virtual void Initialize(float lifetime, float width, float widthMultiplier, float minWidth, int branchTries, float minSpreadDistance, float minSpreadDistanceMultiplier, float maxSpreadDistance, float maxSpreadDistanceMultiplier, float minSpreadAngle, float minSpreadAngleMultiplier, float maxSpreadAngle, float maxSpreadAngleMultiplier, float spreadChance, float spreadChanceMultiplier, float minBranchDistance, float minBranchDistanceMultiplier, float maxBranchDistance, float maxBranchDistanceMultiplier, float minBranchAngle, float minBranchAngleMultiplier, float maxBranchAngle, float maxBranchAngleMultiplier, float branchChance, float branchChanceMultiplier, ref List<Target> ignoredTargets, out List<Arc> arcs, ArcSegment previousArcSegment = null, bool isMainArc = true, int recursion = 0, int maxRecursion = 10)
        {
            this.minSpreadDistance = minSpreadDistance;
            this.minSpreadDistanceMultiplier = minSpreadDistanceMultiplier;

            this.maxSpreadDistance = maxSpreadDistance;
            this.maxSpreadDistanceMultiplier = maxSpreadDistanceMultiplier;

            this.minSpreadAngle = minSpreadAngle;
            this.minSpreadAngleMultiplier = minSpreadAngleMultiplier;

            this.maxSpreadAngle = maxSpreadAngle;
            this.maxSpreadAngleMultiplier = maxSpreadAngleMultiplier;

            this.spreadChance = spreadChance;
            this.spreadChanceMultiplier = spreadChanceMultiplier;


            this.minBranchDistance = minBranchDistance;
            this.minBranchDistanceMultiplier = minBranchDistanceMultiplier;

            this.maxBranchDistance = maxBranchDistance;
            this.maxBranchDistanceMultiplier = maxBranchDistanceMultiplier;

            this.minBranchAngle = minBranchAngle;
            this.minBranchAngleMultiplier = minBranchAngleMultiplier;

            this.maxBranchAngle = maxBranchAngle;
            this.maxBranchAngleMultiplier = maxBranchAngleMultiplier;

            this.branchChance = branchChance;
            this.branchChanceMultiplier = branchChanceMultiplier;

            targeting.radius = maxSpreadDistance;
            targeting.IgnoredTargets = ignoredTargets;

            targets = new List<Target>();
            arcSegments = new List<ArcSegment>();
            this.arcs = new List<Arc>();
            arcs = new List<Arc>();
            this.lifetime = 0.0f;
            this.maxLifetime = lifetime;
            this.width = width;
            this.widthMultiplier = widthMultiplier;
            this.minWidth = minWidth;
            if (width < minWidth)
            {
                Kill();
                return;
            }
            this.branchTries = branchTries;
            this.arcSegment = previousArcSegment;
            this.recursion = recursion;
            this.maxRecursion = maxRecursion;
            this.isMainArc = isMainArc;
            bool spreaded = false;
            if (UnityEngine.Random.Range(0.0f, 1.0f) <= spreadChance && isMainArc && recursion < maxRecursion)
            {
                Target spreadTarget = GetSpreadTarget();
                if (spreadTarget != null)
                {
                    spreaded = true;
                    ignoredTargets.Add(spreadTarget);
                    targets.Add(spreadTarget);
                    CreateNewSegment(spreadTarget, out List<Arc> spreadSubArcs, true);
                    this.arcs.AddRange(spreadSubArcs);
                }
            }

            int branchesToSpawn = branchTries;
            int branchesSpawned = 0;
            bool branched = false;
            do
            {
                if (UnityEngine.Random.Range(0.0f, 1.0f) <= branchChance && previousArcSegment != null && recursion < maxRecursion)
                {
                    Target branchTarget = GetBranchtarget();
                    if (branchTarget != null)
                    {
                        branched = true;
                        branchesSpawned++;
                        ignoredTargets.Add(branchTarget);
                        targets.Add(branchTarget);
                        CreateNewSegment(branchTarget, out List<Arc> branchSubArcs, false);
                        this.arcs.AddRange(branchSubArcs);
                    }
                }
                branchesToSpawn--;
            } while (branchesToSpawn > 0);

            arcs = this.arcs;
            arcs.Add(this);

            if (!spreaded && !branched)
            {
                Kill();
                return;
            }
        }

        protected override void OnPauseableUpdate()
        {
            lifetime += Time.deltaTime;
            if (lifetime > maxLifetime)
            {
                Kill();
            }
        }

        protected void CreateNewSegment(Target target, out List<Arc> subArcs, bool isMainSegment)
        {
            subArcs = new List<Arc>();

            GameObject newSegmentObject = new GameObject();
            if (isMainSegment)
            {
                newSegmentObject.name = "Spread Arc";
            }
            else
            {
                newSegmentObject.name = "Branch Arc";
                widthMultiplier *= 0.5f;
            }
            newSegmentObject.transform.parent = transform;
            if (arcSegment != null)
            {
                newSegmentObject.transform.position = arcSegment.endPos;
            }
            else
            {
                newSegmentObject.transform.position = transform.position;
            }

            ArcSegment newSegment = newSegmentObject.AddComponent<ArcSegment>();
            newSegment.Initialize(newSegment.transform.position, target.transform.position, width);
            arcSegments.Add(newSegment);

            nextArc = newSegmentObject.AddComponent<Arc>();
            nextArc.Initialize(
                maxLifetime, width * widthMultiplier, widthMultiplier, minWidth, branchTries,
                minSpreadDistance * minSpreadDistanceMultiplier, minSpreadDistanceMultiplier,
                maxSpreadDistance * maxSpreadDistanceMultiplier, maxSpreadDistanceMultiplier,
                minSpreadAngle * minSpreadAngleMultiplier, minSpreadAngleMultiplier,
                maxSpreadAngle * maxSpreadAngleMultiplier, maxSpreadAngleMultiplier,
                spreadChance * spreadChanceMultiplier, spreadChanceMultiplier,
                minBranchDistance * minBranchDistanceMultiplier, minBranchDistanceMultiplier,
                maxBranchDistance * maxBranchDistanceMultiplier, maxBranchDistanceMultiplier,
                minBranchAngle * minBranchAngleMultiplier, minBranchAngleMultiplier,
                maxBranchAngle * maxBranchAngleMultiplier, maxBranchAngleMultiplier,
                branchChance * branchChanceMultiplier, branchChanceMultiplier,
                ref targeting.IgnoredTargets, out List<Arc> nextArcSubArcs, newSegment, isMainSegment);
            if (nextArcSubArcs != null)
            {
                subArcs.AddRange(nextArcSubArcs);
            }
        }

        protected Target GetSpreadTarget()
        {
            List<Target> closestTargets = targeting.ClosestTargets;
            if (closestTargets != null && closestTargets.Count > 0)
            {
                foreach (Target target in closestTargets)
                {
                    if (CanSpread(target))
                    {
                        return target;
                    }
                }
            }
            return null;
        }

        protected Target GetBranchtarget()
        {
            List<Target> closestTargets = targeting.ClosestTargets;
            if (closestTargets != null && closestTargets.Count > 0)
            {
                foreach (Target target in closestTargets)
                {
                    if (CanBranch(target))
                    {
                        return target;
                    }
                }
            }
            return null;
        }

        protected bool CanBranch(Target target)
        {
            if (arcSegment == null)
            {
                return true;
            }
            if (Vector2.Distance(target.transform.position, transform.position) >= minBranchDistance)
            {
                if (Vector2.Distance(target.transform.position, transform.position) <= maxBranchDistance)
                {
                    float angle = Vector2.Angle((arcSegment.endPos - arcSegment.startPos).normalized, (target.transform.position - arcSegment.endPos).normalized);
                    if (angle >= minBranchAngle && angle <= maxBranchAngle)
                    {
                        return true;
                    } 
                }
            }
            return false;
        }

        protected bool CanSpread(Target target)
        {
            if (arcSegment == null)
            {
                return true;
            }
            if (Vector2.Distance(target.transform.position, transform.position) >= minSpreadDistance)
            {
                if (Vector2.Distance(target.transform.position, transform.position) <= maxSpreadDistance)
                {
                    float angle = Vector2.Angle((arcSegment.endPos - arcSegment.startPos).normalized, (target.transform.position - arcSegment.endPos).normalized);
                    if (angle >= minSpreadAngle && angle <= maxSpreadAngle)
                    {
                        return true;
                    } 
                }
            }
            return false;
        }

        public void Kill()
        {
            Destroy(gameObject);
        }
    } 
}
