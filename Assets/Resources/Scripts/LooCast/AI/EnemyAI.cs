using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.AI
{
    using Core;
    using Random;
    using Movement;
    using Player;
    using StateMachine;

    public class EnemyAI : ExtendedMonoBehaviour
    {
        public enum State
        {
            Roaming,
            Chasing
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

                if (Vector3.Distance(enemyAI.transform.position, enemyAI.player.transform.position) <= enemyAI.detectionRange)
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

            public Chasing(EnemyAI enemyAI) : base(State.Chasing)
            {
                this.enemyAI = enemyAI;
            }

            public override void Update()
            {
                enemyAI.movement.AccelerateToPosition(enemyAI.player.transform.position);

                if (Vector3.Distance(enemyAI.transform.position, enemyAI.player.transform.position) > enemyAI.detectionRange)
                {
                    enemyAI.finiteStateMachine.SetCurrentState(State.Roaming);
                }
            }
        }

        [SerializeField] private float minRoamingDistance;
        [SerializeField] private float maxRoamingDistance;
        [SerializeField] private float roamingReachedDestinationDistance;
        [SerializeField] private float roamingSpeedMultiplier;
        [SerializeField] private float detectionRange;
        [SerializeField] private LayerMask enemyLayerMask;

        private FiniteStateMachine<State> finiteStateMachine = new FiniteStateMachine<State>();
        private IMovement movement;
        private Player player;

        private void Awake()
        {
            movement = GetComponent<IMovement>();
            player = FindObjectOfType<Player>();
        }

        private void Start()
        {
            finiteStateMachine.Add(new Roaming(this));
            finiteStateMachine.Add(new Chasing(this));

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
