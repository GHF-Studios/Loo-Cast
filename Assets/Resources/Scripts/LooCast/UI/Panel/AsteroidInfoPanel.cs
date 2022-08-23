using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Panel
{
    using LooCast.Asteroid;

    public class AsteroidInfoPanel : MonoBehaviour
    {
        public Asteroid CurrentAsteroid
        {
            get
            {
                return currentAsteroid;
            }
            set
            {
                currentAsteroid = value;
                Refresh();
            }
        }
        private Asteroid currentAsteroid;
        [SerializeField] private Text sizeValue;
        [SerializeField] private Text rarityValue;
        [SerializeField] private Text massValue;

        private void Update()
        {
            if (enabled)
            {
                Refresh();
            }
        }

        public void Refresh()
        {
            sizeValue.text = Enum.GetName(typeof(Asteroid.Size), CurrentAsteroid.AsteroidSize);
            rarityValue.text = Enum.GetName(typeof(Asteroid.Rarity), CurrentAsteroid.AsteroidRarity);
            massValue.text = string.Format("{0:n0}", CurrentAsteroid.Mass) + "t";
        }
    }
}