using UnityEngine;

namespace LooCast.Movement.Effect
{
    [RequireComponent(typeof(IMovement))]
    public abstract class MovementEffect : MonoBehaviour
    {
        public IMovement Movement;

        private void Start()
        {
            Movement = GetComponent<IMovement>();
        }
    }
}
