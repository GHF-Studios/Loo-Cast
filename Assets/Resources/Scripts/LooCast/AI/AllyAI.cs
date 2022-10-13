using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.AI
{
    using Core;
    using Random;
    using Movement;
    using StateMachine;

    public class AllyAI : ExtendedMonoBehaviour
    {
        public enum State
        {
            Roaming,
            Evade
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
                speedMultiplier = allyAI.movement.Speed.AddPermanentMultiplier(0.15f);
                startingPosition = allyAI.transform.position;
                roamingPosition = GetRoamingPosition();
            }

            public override void Exit()
            {
                allyAI.movement.Speed.RemovePermanentMultiplier(speedMultiplier);
            }

            public override void PauseableUpdate()
            {
                allyAI.movement.AccelerateToPosition(roamingPosition);

                if (Vector3.Distance(allyAI.transform.position, roamingPosition) < allyAI.roamingReachedDestinationDistance)
                {
                    roamingPosition = GetRoamingPosition();
                }

                if (Physics2D.OverlapCircle(allyAI.transform.position, allyAI.detectionRange, allyAI.enemyLayerMask))
                {
                    allyAI.finiteStateMachine.SetCurrentState(State.Evade);
                }
            }

            private Vector3 GetRoamingPosition()
            {
                return startingPosition + Random.Direction() * Random.Range(allyAI.minRoamingDistance, allyAI.maxRoamingDistance);
            }
        }

        public class Evade : State<State>
        {
            private AllyAI allyAI;

            public Evade(AllyAI allyAI) : base(State.Evade)
            {
                this.allyAI = allyAI;
            }

            public override void PauseableUpdate()
            {
                Collider2D[] enemyColliders = Physics2D.OverlapCircleAll(allyAI.transform.position, allyAI.detectionRange, allyAI.enemyLayerMask);
                if (enemyColliders.Length > 0)
                {
                    allyAI.movement.AccelerateInDirection(GetEvadeDirection(enemyColliders));
                }
                else
                {
                    allyAI.finiteStateMachine.SetCurrentState(State.Roaming);
                }
            }

            private Vector3 GetEvadeDirection(Collider2D[] enemyColliders)
            {
                List<Vector3> evadeDirectionList = new List<Vector3>();
                foreach (Collider2D enemyCollider in enemyColliders)
                {
                    float evadeDirectionWeight = Mathf.Pow(allyAI.detectionRange / Vector2.Distance(allyAI.transform.position, enemyCollider.transform.position), 2);
                    Vector3 weightedEvadeDirection = (allyAI.transform.position - enemyCollider.transform.position).normalized * evadeDirectionWeight;
                    evadeDirectionList.Add(weightedEvadeDirection);
                }
                return evadeDirectionList.Aggregate(new Vector3(0, 0, 0), (sumVector, currentVector) => sumVector + currentVector) / evadeDirectionList.Count;
            }
        }

        [SerializeField] private float minRoamingDistance;
        [SerializeField] private float maxRoamingDistance;
        [SerializeField] private float roamingReachedDestinationDistance;
        [SerializeField] private float detectionRange;
        [SerializeField] private LayerMask enemyLayerMask;

        private FiniteStateMachine<State> finiteStateMachine = new FiniteStateMachine<State>();
        private IMovement movement;

        private void Awake()
        {
            movement = GetComponent<IMovement>();
        }

        private void Start()
        {
            finiteStateMachine.Add(new Roaming(this));
            finiteStateMachine.Add(new Evade(this));

            finiteStateMachine.SetCurrentState(State.Roaming);
        }

        private void Update()
        {
            finiteStateMachine.Update();
        }

        private void FixedUpdate()
        {
            finiteStateMachine.FixedUpdate();
        }

        protected override void PauseableUpdate()
        {
            finiteStateMachine.PauseableUpdate();
        }

        protected override void PauseableFixedUpdate()
        {
            finiteStateMachine.PauseableFixedUpdate();
        }
    } 
}
