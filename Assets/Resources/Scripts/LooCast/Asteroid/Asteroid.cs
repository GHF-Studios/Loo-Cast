using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Asteroid
{
    using Data;
    using Data.Runtime;

    public class Asteroid : MonoBehaviour
    {
        #region Data
        public AsteroidData Data;
        public AsteroidRuntimeSet RuntimeSet;
        #endregion

        #region Fields
        private MeshFilter meshFilter;
        private MeshCollider meshCollider;
        private Rigidbody rigidbody;

        private Vector3 randomRotation;
        private Vector3 randomSpeed;
        #endregion

        private void Start()
        {
            RuntimeSet.Add(this);

            rigidbody = GetComponent<Rigidbody>();
            meshFilter = GetComponent<MeshFilter>();
            meshCollider = GetComponent<MeshCollider>();

            randomRotation = new Vector3
                (
                Data.angularSpeedCurve.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)),
                Data.angularSpeedCurve.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)),
                Data.angularSpeedCurve.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f))
                );
            randomSpeed = new Vector3
                (
                Data.speedCurve.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)),
                Data.speedCurve.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f)),
                0.0f
                );
            float randomScale = Data.scaleCurve.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float density = 1000.0f;
            Mesh randomMesh = Data.Meshes[UnityEngine.Random.Range(0, Data.Meshes.Length - 1)];

            rigidbody.velocity = randomSpeed * (1 / randomScale);
            rigidbody.angularVelocity = randomRotation * (1 / randomScale);
            rigidbody.mass = Mathf.Pow(randomScale, 3) * density;
            transform.localScale = Vector3.one * randomScale;
            meshFilter.mesh = randomMesh;
            meshCollider.sharedMesh = randomMesh;

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
