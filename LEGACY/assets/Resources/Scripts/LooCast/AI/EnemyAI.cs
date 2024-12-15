using UnityEngine;
using System.Linq;

namespace LooCast.AI
{
    using LooCast.Core;
    using LooCast.System;
    using LooCast.Random;
    using LooCast.Movement;
    using LooCast.StateMachine;
    using LooCast.Util;
    using LooCast.Target;
    using LooCast.Game;

    public class EnemyAI : ExtendedMonoBehaviour
    {
        public enum State
        {
            Roaming,
            Chasing,
            Retreating
        }

        public class Roaming : State<State>
        {
            private EnemyAI enemyAI;
            private Variable.Multiplier speedMultiplier;
            private Vector3 startingPosition;
            private Vector3 roamingPosition;

            public Roaming(EnemyAI enemyAI) : base(State.Roaming)
            {
                this.enemyAI = enemyAI;
            }

            public override void Enter()
            {
                speedMultiplier = enemyAI.movement.Speed.AddPermanentMultiplier(enemyAI.roamingSpeedMultiplier);
                startingPosition = enemyAI.transform.position;
                roamingPosition = GetRoamingPosition();
            }

            public override void Exit()
            {
                enemyAI.movement.Speed.RemovePermanentMultiplier(speedMultiplier);
            }

            public override void Update()
            {
                enemyAI.movement.AccelerateToPosition(roamingPosition);
                if (Vector3.Distance(enemyAI.transform.position, roamingPosition) <= enemyAI.roamingReachedDestinationDistance)
                {
                    roamingPosition = GetRoamingPosition();
                }

                Target target = TargetingUtil.GetTargetInRadius(enemyAI.transform.position, enemyAI.detectionRange, enemyAI.enemyLayerMask);
                if (target != null)
                {
                    enemyAI.finiteStateMachine.SetCurrentState(State.Chasing);
                }
            }

            private Vector3 GetRoamingPosition()
            {
                return startingPosition + Random.Direction() * Random.Range(enemyAI.minRoamingDistance, enemyAI.maxRoamingDistance);
            }
        }

        public class Chasing : State<State>
        {
            private EnemyAI enemyAI;
            private Target lockedTarget;

            public Chasing(EnemyAI enemyAI) : base(State.Chasing)
            {
                this.enemyAI = enemyAI;
            }

            public override void Enter()
            {
                lockedTarget = GetClosestTarget(GetTargets());
            }

            public override void Update()
            {
                enemyAI.movement.AccelerateToPosition(lockedTarget.Transform.position);

                Target[] targets = GetTargets();
                if (targets.Length > 0)
                {
                    if (!targets.Contains(lockedTarget))
                    {
                        lockedTarget = GetClosestTarget(targets);
                    }
                }
                else
                {
                    enemyAI.finiteStateMachine.SetCurrentState(State.Roaming);
                }
            }

            private Target[] GetTargets()
            {
                return TargetingUtil.GetTargetsInRadius(enemyAI.transform.position, enemyAI.detectionRange, enemyAI.enemyLayerMask);
            }

            private Target GetClosestTarget(Target[] targets)
            {
                targets = TargetingUtil.SortTargets(targets, enemyAI.transform.position, TargetingUtil.SortingType.Closest);
                return targets[0];
            }
        }

        public class Retreating : State<State>
        {
            private EnemyAI enemyAI;

            public Retreating(EnemyAI enemyAI) : base(State.Retreating)
            {
                this.enemyAI = enemyAI;
            }

            public override void Update()
            {
                enemyAI.movement.AccelerateToPosition(enemyAI.roamingRootPosition);

                if (Vector3.Distance(enemyAI.transform.position, enemyAI.roamingRootPosition) <= enemyAI.roamingReachedDestinationDistance)
                {
                    enemyAI.finiteStateMachine.SetCurrentState(State.Roaming);
                }
            }
        }

        [SerializeField] private Vector3 roamingRootPosition;
        [SerializeField] private float minRoamingDistance;
        [SerializeField] private float maxRoamingDistance;
        [SerializeField] private float roamingReachedDestinationDistance;
        [SerializeField] private float roamingSpeedMultiplier;
        [SerializeField] private float detectionRange;
        [SerializeField] private LayerMask enemyLayerMask;

        private FiniteStateMachine<State> finiteStateMachine = new FiniteStateMachine<State>();
        private IMovement movement;

        private void Start()
        {
            movement = GetComponent<IMovement>();

            roamingRootPosition = transform.position;

            finiteStateMachine.Add(new Roaming(this));
            finiteStateMachine.Add(new Chasing(this));
            finiteStateMachine.Add(new Retreating(this));

            finiteStateMachine.SetCurrentState(State.Roaming);
        }

        protected override void PauseableUpdate()
        {
            finiteStateMachine.Update();
        }

        protected override void PauseableFixedUpdate()
        {
            finiteStateMachine.FixedUpdate();
        }
    } 
}
