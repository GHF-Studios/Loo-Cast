using UnityEngine;

namespace LooCast.Movement.Effect
{
    [RequireComponent(typeof(IMovement))]
    public abstract class MovementEffect : MonoBehaviour
    {
        public IMovement Movement;

        public void Initialize()
        {
            Movement = GetComponent<IMovement>();
        }
    }
}
