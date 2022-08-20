using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Asteroid
{
    using Data;
    using Data.Runtime;
    using LooCast.Chance;
    using LooCast.Resource;

    public class Asteroid : MonoBehaviour
    {
        #region Data
        public AsteroidData Data;
        public AsteroidRuntimeSet RuntimeSet;
        #endregion

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

        #region Structs
        public struct ResourceDeposit
        {
            public Resource Resource { get; private set; }
            public float Deposit { get; set; }

            public ResourceDeposit(Resource resource, float deposit)
            {
                Resource = resource;
                Deposit = deposit;
            }
        }
        #endregion

        #region Properties
        public Size AsteroidSize {get; private set;}
        public Rarity AsteroidRarity {get; private set;}
        public ResourceDeposit[] ResourceDeposits {get; private set;}
        #endregion

        #region Fields
        private MeshFilter meshFilter;
        private MeshRenderer meshRenderer;
        private MeshCollider meshCollider;
        private Rigidbody rigidbody;
        #endregion

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

            ResourceDeposits = new ResourceDeposit[asteroidRarityData.Resources.Length];
            for (int i = 0; i < ResourceDeposits.Length; i++)
            {
                ResourceDeposits[i] = new ResourceDeposit(asteroidRarityData.Resources[i], asteroidRarityData.Deposits[i].Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)));
            }

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
    } 
}
