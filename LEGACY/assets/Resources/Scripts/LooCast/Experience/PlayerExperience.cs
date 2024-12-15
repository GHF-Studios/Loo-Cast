using UnityEngine;

namespace LooCast.Experience
{
    using LooCast.System;
    using Data;
    using Data.Runtime;
    using LooCast.Core;
    using Variable;
    using Attribute.Stat;
    using Currency;
    using Sound;

    [DisallowMultipleComponent]
    public class PlayerExperience : ExtendedMonoBehaviour, IExperience
    {
        public PlayerExperienceData Data;
        public PlayerExperienceRuntimeData RuntimeData;
        public Stats Stats;
        public Coins Coins;
        public Tokens Tokens;

        private GameSoundHandler soundHandler;

        private void Start()
        {
            RuntimeData.Initialize(Data);

            soundHandler = FindObjectOfType<GameSoundHandler>();
        }

        public void AddExperience(float xp)
        {
            Coins.Balance.Value += Mathf.RoundToInt(xp);
            RuntimeData.CurrentExperience.BaseValue += xp;

            UpdateLevelProgress(RuntimeData.CurrentExperience.Value);
        }

        private void UpdateLevelProgress(float overflowXP)
        {
            if (overflowXP == RuntimeData.LevelExperienceMax.Value)
            {
                IncreaseLevel();
                RuntimeData.CurrentExperience.BaseValue = 0;
                return;
            }

            if (overflowXP > RuntimeData.LevelExperienceMax.Value)
            {
                UpdateLevelProgress(overflowXP - RuntimeData.LevelExperienceMax.Value);
                IncreaseLevel();
                return;
            }

            if (overflowXP < RuntimeData.LevelExperienceMax.Value)
            {
                RuntimeData.CurrentExperience.BaseValue = overflowXP;
                return;
            }
        }

        private void IncreaseLevel()
        {
            RuntimeData.CurrentLevel.Value++;
            RuntimeData.LevelExperienceMax.BaseValue *= Stats.LevelExperienceMaxMultiplier;
            Tokens.Balance.Value++;
            soundHandler.SoundUpgrade();
        }
    } 
}