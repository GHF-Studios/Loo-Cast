using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.AI
{
    using Core;
    using Random;
    using Movement;
    using StateMachine;
    using Util;
    using Enemy;
    using Target;

    public class AllyAI : Component
    {
        public enum State
        {
            Roaming,
            Evading,
            Retreating
        }

        public class Roaming : State<State>
        {
            private AllyAI allyAI;
            private Variable.Multiplier speedMultiplier;
            private Vector3 startingPosition;
            private Vector3 roamingPosition;

            public Roaming(AllyAI allyAI) : base(State.Roaming)
            {
                this.allyAI = allyAI;
            }

            public override void Enter()
            {
                speedMultiplier = allyAI.movement.Speed.AddPermanentMultiplier(allyAI.roamingSpeedMultiplier);
                startingPosition = allyAI.transform.position;
                roamingPosition = GetRoamingPosition();
            }

            public override void Exit()
            {
                allyAI.movement.Speed.RemovePermanentMultiplier(speedMultiplier);
            }

            public override void Update()
            {
                allyAI.movement.AccelerateToPosition(roamingPosition);

                if (Vector3.Distance(allyAI.transform.position, roamingPosition) <= allyAI.roamingReachedDestinationDistance)
                {
                    roamingPosition = GetRoamingPosition();
                }

                Target target = TargetingUtil.GetTargetInRadius(allyAI.transform.position, allyAI.detectionRange, allyAI.enemyLayerMask);
                if (target != null)
                {
                    allyAI.finiteStateMachine.SetCurrentState(State.Evading);
                }
            }

            private Vector3 GetRoamingPosition()
            {
                return startingPosition + Random.Direction() * Random.Range(allyAI.minRoamingDistance, allyAI.maxRoamingDistance);
            }
        }

        public class Evading : State<State>
        {
            private AllyAI allyAI;

            public Evading(AllyAI allyAI) : base(State.Evading)
            {
                this.allyAI = allyAI;
            }

            public override void Update()
            {
                Target[] enemies = TargetingUtil.GetRandomProximityTargets(allyAI.transform.position, allyAI.detectionRange, allyAI.enemyLayerMask);
                if (enemies.Length > 0)
                {
                    allyAI.movement.AccelerateInDirection(GetEvadeDirection(enemies));
                }
                else
                {
                    allyAI.finiteStateMachine.SetCurrentState(State.Retreating);
                }
            }

            private Vector3 GetEvadeDirection(Target[] enemies)
            {
                List<Vector3> evadeDirectionList = new List<Vector3>();
                foreach (Target enemy in enemies)
                {
                    float evadeDirectionWeight = Mathf.Pow(allyAI.detectionRange / Vector2.Distance(allyAI.transform.position, enemy.Transform.position), 2);
                    Vector3 weightedEvadeDirection = (allyAI.transform.position - enemy.Transform.position).normalized * evadeDirectionWeight;
                    evadeDirectionList.Add(weightedEvadeDirection);
                }
                return (evadeDirectionList.Aggregate(new Vector3(0, 0, 0), (sumVector, currentVector) => sumVector + currentVector) / evadeDirectionList.Count).normalized;
            }
        }

        public class Retreating : State<State>
        {
            private AllyAI allyAI;

            public Retreating(AllyAI allyAI) : base(State.Retreating)
            {
                this.allyAI = allyAI;
            }

            public override void Update()
            {
                allyAI.movement.AccelerateToPosition(allyAI.roamingRootPosition);

                if (Vector3.Distance(allyAI.transform.position, allyAI.roamingRootPosition) <= allyAI.roamingReachedDestinationDistance)
                {
                    allyAI.finiteStateMachine.SetCurrentState(State.Roaming);
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
            finiteStateMachine.Add(new Evading(this));
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
