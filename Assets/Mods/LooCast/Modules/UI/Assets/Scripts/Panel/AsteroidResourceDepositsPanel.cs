using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Panel
{
    using LooCast.Asteroid;
    using LooCast.Resource;

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
                if (value == null)
                {
                    currentAsteroid = null;
                    return;
                }
                else
                {
                    if (resourceDepositLabelParent.childCount < value.ResourceDeposits.Length)
                    {
                        int amountNewDeposits = value.ResourceDeposits.Length - resourceDepositLabelParent.childCount;
                        for (int i = 0; i < amountNewDeposits; i++)
                        {
                            GameObject resourceDepositLabel = Instantiate(textPrefab, resourceDepositLabelParent);
                            GameObject resourceDepositValue = Instantiate(textPrefab, resourceDepositValueParent);
                        }
                    }
                    else
                    {
                        int amountOldDeposits = resourceDepositLabelParent.childCount - value.ResourceDeposits.Length;
                        for (int i = 0; i < amountOldDeposits; i++)
                        {
                            Destroy(resourceDepositLabelParent.GetChild(i).gameObject);
                            Destroy(resourceDepositValueParent.GetChild(i).gameObject);
                        }
                    }
                }

                currentAsteroid = value;

                //Update Resource Deposit Labels & Values
                for (int i = 0; i < currentAsteroid.ResourceDeposits.Length; i++)
                {
                    resourceDepositLabelParent.GetChild(i).gameObject.name = currentAsteroid.ResourceDeposits[i].Resource.ResourceName;
                    resourceDepositValueParent.GetChild(i).gameObject.name = currentAsteroid.ResourceDeposits[i].Resource.ResourceName;
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