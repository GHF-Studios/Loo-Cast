using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute
{
    using LooCast.Attribute.Stat;
    using LooCast.Data;
    using LooCast.Util;

    [CreateAssetMenu(fileName = "Attributes", menuName = "Data/Attribute/Attributes", order = 0)]
    public class Attributes : ScriptableObject
    {
        public Stats Stats;

        public CharismaAttribute Charisma;
        public ConstitutionAttribute Constitution;
        public DefenseAttribute Defense;
        public DexterityAttribute Dexterity;
        public IntelligenceAttribute Intelligence;
        public LuckAttribute Luck;
        public PerceptionAttribute Perception;
        public StrengthAttribute Strength;
        public WillpowerAttribute Willpower;
        public WisdomAttribute Wisdom;

        private void OnEnable()
        {
            LoadAttribute(Charisma);
            LoadAttribute(Constitution);
            LoadAttribute(Defense);
            LoadAttribute(Dexterity);
            LoadAttribute(Intelligence);
            LoadAttribute(Luck);
            LoadAttribute(Perception);
            LoadAttribute(Strength);
            LoadAttribute(Willpower);
            LoadAttribute(Wisdom);
        }

        private void OnDisable()
        {
            SaveAttribute(Charisma);
            SaveAttribute(Constitution);
            SaveAttribute(Defense);
            SaveAttribute(Dexterity);
            SaveAttribute(Intelligence);
            SaveAttribute(Luck);
            SaveAttribute(Perception);
            SaveAttribute(Strength);
            SaveAttribute(Willpower);
            SaveAttribute(Wisdom);
        }

        public void Cheat()
        {
            Charisma.Level.Value = Charisma.MaxLevel.Value;
            Constitution.Level.Value = Constitution.MaxLevel.Value;
            Defense.Level.Value = Defense.MaxLevel.Value;
            Dexterity.Level.Value = Dexterity.MaxLevel.Value;
            Intelligence.Level.Value = Intelligence.MaxLevel.Value;
            Luck.Level.Value = Luck.MaxLevel.Value;
            Perception.Level.Value = Perception.MaxLevel.Value;
            Strength.Level.Value = Strength.MaxLevel.Value;
            Willpower.Level.Value = Willpower.MaxLevel.Value;
            Wisdom.Level.Value = Wisdom.MaxLevel.Value;
        }

        public void Uncheat()
        {
            Charisma.Level.Value = 0;
            Constitution.Level.Value = 0;
            Defense.Level.Value = 0;
            Dexterity.Level.Value = 0;
            Intelligence.Level.Value = 0;
            Luck.Level.Value = 0;
            Perception.Level.Value = 0;
            Strength.Level.Value = 0;
            Willpower.Level.Value = 0;
            Wisdom.Level.Value = 0;
        }

        public Attribute GetAttribute(string attributeName)
        {
            switch (attributeName)
            {
                case "Charisma": return Charisma;
                case "Constitution": return Constitution;
                case "Defense": return Defense;
                case "Dexterity": return Dexterity;
                case "Intelligence": return Intelligence;
                case "Luck": return Luck;
                case "Perception": return Perception;
                case "Strength": return Strength;
                case "Willpower": return Willpower;
                case "Wisdom": return Wisdom;
                default: throw new ArgumentException("Invalid attribute name!");
            }
        }

        public void SaveAttribute(Attribute attribute, bool saveDefault = false)
        {
            JSONUtil.SaveData(new Attribute.DataContainer(attribute.Stats, attribute.Level, attribute.MaxLevel, attribute.ProposedLevelChange), $"{(saveDefault ? "Default/" : "")}Attribute/{attribute.AttributeName}.json");
        }

        public void LoadAttribute(Attribute attribute)
        {
            Attribute.DataContainer dataContainer = JSONUtil.LoadData<Attribute.DataContainer>($"Attribute/{attribute.AttributeName}.json");
            attribute.Stats = dataContainer.GetStats(Stats);
            attribute.Level = dataContainer.GetLevel();
            attribute.MaxLevel = dataContainer.GetMaxLevel();
            attribute.ProposedLevelChange = dataContainer.GetProposedLevelChange();
        }
    } 
}
