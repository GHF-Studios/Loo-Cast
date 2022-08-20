using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Asteroid
{
    using Data;
    using Data.Runtime;
    using LooCast.Chance;

    public class Asteroid : MonoBehaviour
    {
        #region Data
        public AsteroidData Data;
        public AsteroidRuntimeSet RuntimeSet;
        #endregion

        #region Classes
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

        #region Properties
        public Size AsteroidSize {get; private set;}
        public Rarity AsteroidRarity {get; private set;}
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

            Vector3 randomRotation = new Vector3(Data.AngularSpeedDistribution.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), Data.AngularSpeedDistribution.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), Data.AngularSpeedDistribution.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)));
            Vector3 randomSpeed = new Vector3(Data.SpeedDistribution.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), Data.SpeedDistribution.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)), 0.0f);
            float randomScale = Data.ScaleDistribution.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float density = 1000.0f;
            AsteroidSize = (Size)Chance.GetRandomWeightedIndex(Data.SizeWeights);
            AsteroidRarity = (Rarity)Chance.GetRandomWeightedIndex(Data.RarityWeights);
            Mesh mesh = Data.Meshes[(int)AsteroidSize];
            Material material = Data.Materials[(int)AsteroidRarity];

            transform.localScale = Vector3.one * 100.0f * randomScale;
            rigidbody.velocity = randomSpeed * (1 / randomScale);
            rigidbody.angularVelocity = randomRotation * (1 / randomScale);
            rigidbody.mass = Mathf.Pow(randomScale, 3) * density;
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
