using UnityEngine;

namespace LooCast.Movement.Effect
{
    [RequireComponent(typeof(Movement))]
    public abstract class MovementEffect : MonoBehaviour
    {
        public Movement Movement;

        private void Start()
        {
            Movement = GetComponent<Movement>();
        }
    }
}
