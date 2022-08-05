using UnityEngine;

namespace LooCast.Experience
{
    public abstract class Experience : MonoBehaviour
    {
        public abstract void AddExperience(float xp);

        protected abstract void UpdateLevelProgress(float overflowXP);

        protected abstract void IncreaseLevel();
    } 
}
