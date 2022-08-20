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
        private Vector3 randomRotation;
        private Vector3 randomSpeed;
        #endregion

        private void Start()
        {
            RuntimeSet.Add(this);

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
            Rigidbody rigidbody = GetComponent<Rigidbody>();
            rigidbody.velocity = randomSpeed;
            rigidbody.angularVelocity = randomRotation;
            transform.localScale = Vector3.one * Data.scaleCurve.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
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
