using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Asteroid
{
    using Data;
    using Data.Runtime;
    using Chance;
    using Resource;
    using Item;
    using Item.Data;

    public class Asteroid : MonoBehaviour
    {
        #region Enums
        public enum Size
        {
            Tiny,
            Small,
            Medium,
            Large,
            Huge
        }
        public enum Rarity
        {
            Common,
            Uncommon,
            Rare,
            Epic,
            Legendary
        }
        #endregion
        
        #region Classes
        public class ResourceDeposit
        {
            public Resource Resource { get; private set; }
            public float Deposit { get; private set; }

            public ResourceDeposit(Resource resource, float deposit)
            {
                Resource = resource;
                Deposit = deposit;
            }

            public bool TryExtractMass(float mass)
            {
                if (mass <= Deposit)
                {
                    Deposit -= mass;
                    return true;
                }
                return false;
            }

            public void DropAll(Vector3 dropPosition, float maxOffsetMagnitude, ItemDatas itemDatas)
            {
                ResourceItemData resourceItemData = (ResourceItemData)itemDatas.GetItemData(Resource.ResourceName);
                float depositToDrop = Deposit;
                while (depositToDrop > resourceItemData.MaxAmount.Value)
                {
                    Drop(resourceItemData.MaxAmount.Value);
                    depositToDrop -= resourceItemData.MaxAmount.Value;
                }
                Drop(depositToDrop);

                void Drop(float dropAmount)
                {
                    Vector3 randomSpawnPositionOffset = new Vector3(UnityEngine.Random.Range(-maxOffsetMagnitude, maxOffsetMagnitude), UnityEngine.Random.Range(-maxOffsetMagnitude, maxOffsetMagnitude));
                    randomSpawnPositionOffset.z = 0.0f;

                    ResourceItem resourceItem = (ResourceItem)resourceItemData.CreateItem();
                    resourceItem.Amount = dropAmount;
                    resourceItem.DropItem(dropPosition + randomSpawnPositionOffset);
                }
            }
        }
        #endregion

        #region Data
        public AsteroidData Data;
        public AsteroidRuntimeSet RuntimeSet;
        #endregion

        #region Properties
        public Size AsteroidSize { get; private set; }
        public Rarity AsteroidRarity { get; private set; }
        public float Mass { get; private set; }
        public ResourceDeposit[] ResourceDeposits { get; private set; }
        #endregion

        #region Fields
        [SerializeField] private ItemDatas itemDatas;

        private MeshFilter meshFilter;
        private MeshRenderer meshRenderer;
        private MeshCollider meshCollider;
        private new Rigidbody rigidbody;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            RuntimeSet.Add(this);

            rigidbody = GetComponent<Rigidbody>();
            meshFilter = GetComponent<MeshFilter>();
            meshRenderer = GetComponent<MeshRenderer>();
            meshCollider = GetComponent<MeshCollider>();
            
            Vector3 randomRotation = new Vector3(Data.AngularSpeed.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), Data.AngularSpeed.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), Data.AngularSpeed.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)));
            Vector3 randomSpeed = new Vector3(Data.Speed.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), Data.Speed.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), 0.0f);
            float randomScale = Data.Scale.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float density = 1000.0f;
            float baseScale = 100.0f;

            transform.localScale = Vector3.one * baseScale * randomScale;
            rigidbody.velocity = randomSpeed * (1 / randomScale);
            rigidbody.angularVelocity = randomRotation * (1 / randomScale);
            rigidbody.mass = Mathf.Pow(randomScale, 3) * density;

            AsteroidSize = (Size)Chance.GetRandomWeightedIndex(Data.SizeWeights);
            AsteroidRarity = (Rarity)Chance.GetRandomWeightedIndex(Data.RarityWeights);

            AsteroidSizeData asteroidSizeData = Data.AsteroidSizeDatas[(int)AsteroidSize];
            AsteroidRarityData asteroidRarityData = Data.AsteroidRarityDatas[(int)AsteroidRarity];

            Mass = asteroidSizeData.Mass.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));

            #region Deposit Creation
            //First we get all the deposit weights and calculate the deposit weight sum
            float depositWeightSum = 0;
            float[] depositWeights = new float[asteroidRarityData.Resources.Length];
            for (int i = 0; i < depositWeights.Length; i++)
            {
                depositWeights[i] = asteroidRarityData.DepositWeights[i].Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
                depositWeightSum += depositWeights[i];
            }

            //Then we calculate the respective fractions of the total mass
            float[] totalMassFractions = new float[depositWeights.Length];
            for (int i = 0; i < totalMassFractions.Length; i++)
            {
                totalMassFractions[i] = depositWeights[i] / depositWeightSum;
            }

            //Then we use the fractions to calculate the actual mass of each deposit
            float[] depositMasses = new float[totalMassFractions.Length];
            for (int i = 0; i < depositMasses.Length; i++)
            {
                depositMasses[i] = Mass * totalMassFractions[i];
            }

            //Finally we actually create the deposits
            ResourceDeposits = new ResourceDeposit[asteroidRarityData.Resources.Length];
            for (int i = 0; i < ResourceDeposits.Length; i++)
            {
                ResourceDeposits[i] = new ResourceDeposit(asteroidRarityData.Resources[i], depositMasses[i]);
            }
            #endregion

            Mesh mesh = asteroidSizeData.Meshes[UnityEngine.Random.Range(0, asteroidSizeData.Meshes.Length - 1)];
            Material material = asteroidRarityData.Material;

            meshFilter.mesh = mesh;
            meshRenderer.material = material;
            meshCollider.sharedMesh = mesh;
        }

        private void Update()
        {
            transform.position = new Vector3(transform.position.x, transform.position.y, 0.0f);
        }

        private void OnDestroy()
        {
            RuntimeSet.Remove(this);
        }
        #endregion

        #region Methods
        public bool TryExtractResource(float extractedMass, Resource resource)
        {
            foreach (ResourceDeposit resourceDeposit in ResourceDeposits)
            {
                if (resourceDeposit.Resource == resource)
                {
                    return resourceDeposit.TryExtractMass(extractedMass);
                }
            }
            return false;
        }

        public void Destroy()
        {
            foreach (ResourceDeposit resourceDeposit in ResourceDeposits)
            {
                if (resourceDeposit.Deposit >= 1.0f)
                {
                    resourceDeposit.DropAll(transform.position, meshFilter.mesh.bounds.max.magnitude, itemDatas); 
                }
            }
            Destroy(gameObject);
        }
        #endregion
    } 
}
