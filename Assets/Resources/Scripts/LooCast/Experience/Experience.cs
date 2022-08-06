using UnityEngine;

namespace LooCast.Experience
{
    using Data;

    public abstract class Experience : MonoBehaviour
    {
        public void Initialize(ExperienceData data)
        {

        }

        public abstract void AddExperience(float xp);

        protected abstract void UpdateLevelProgress(float overflowXP);

        protected abstract void IncreaseLevel();
    } 
}
