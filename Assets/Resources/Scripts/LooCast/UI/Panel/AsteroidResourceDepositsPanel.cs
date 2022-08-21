using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Panel
{
    using LooCast.Asteroid;

    public class AsteroidResourceDepositsPanel : MonoBehaviour
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

                //Destroy old Resource Deposit Labels
                for (int i = 0; i < resourceDepositLabelParent.childCount; i++)
                {
                    Destroy(resourceDepositLabelParent.GetChild(i).gameObject);
                }

                //Destroy old Resource Deposit Values
                for (int i = 0; i < resourceDepositValueParent.childCount; i++)
                {
                    Destroy(resourceDepositValueParent.GetChild(i).gameObject);
                }

                //Create new Resource Deposit Labels & Values
                for (int i = 0; i < currentAsteroid.ResourceDeposits.Length; i++)
                {
                    GameObject resourceDepositLabel = Instantiate(textPrefab, resourceDepositLabelParent);
                    resourceDepositLabel.name = currentAsteroid.ResourceDeposits[i].Resource.ResourceName;

                    GameObject resourceDepositValue = Instantiate(textPrefab, resourceDepositValueParent);
                    resourceDepositValue.name = currentAsteroid.ResourceDeposits[i].Resource.ResourceName;
                }

                Refresh();
            }
        }
        private Asteroid currentAsteroid;

        [SerializeField] private Transform resourceDepositLabelParent;
        [SerializeField] private Transform resourceDepositValueParent;
        [SerializeField] private GameObject textPrefab;

        private void Update()
        {
            if (enabled)
            {
                Refresh();
            }
        }

        public void Refresh()
        {
            //Refresh Resource Deposit Labels
            for (int i = 0; i < CurrentAsteroid.ResourceDeposits.Length; i++)
            {
                Transform resourceDepositLabelTransform = resourceDepositLabelParent.GetChild(i);
                Text resourceDepositLabel = resourceDepositLabelTransform.gameObject.GetComponent<Text>();
                resourceDepositLabel.text = CurrentAsteroid.ResourceDeposits[i].Resource.ResourceName;
            }

            //Refresh Resource Deposit Values
            for (int i = 0; i < CurrentAsteroid.ResourceDeposits.Length; i++)
            {
                Transform resourceDepositValueTransform = resourceDepositValueParent.GetChild(i);
                Text resourceDepositValue = resourceDepositValueTransform.gameObject.GetComponent<Text>();
                resourceDepositValue.text = string.Format("{0:n0}", CurrentAsteroid.ResourceDeposits[i].Deposit) + "t";
            }
        }
    }
}