using System;
using UnityEngine;

namespace LooCast.Util
{
    public class ParallelProcessorUtil : MonoBehaviour
    {
        #region Classes
        public class ParallelProcessor
        {
            public ComputeShader ComputeShader => computeShader;

            [SerializeField] private ComputeShader computeShader;

            public ParallelProcessor(ComputeShader computeShader)
            {
                this.computeShader = computeShader;
            }

            public void Process()
            {
                
            }
        }
        #endregion

        #region Static Properties
        public static ParallelProcessorUtil Instance { get; private set; }
        #endregion

        #region Static Fields
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            if (Instance != null)
            {
                Destroy(gameObject);
                return;
            }

            Instance = this;
            DontDestroyOnLoad(gameObject);
        }
        #endregion

        #region Static Methods
        public static ParallelProcessor CreateParallelProcessor(ComputeShader computeShader)
        {
            return new ParallelProcessor(computeShader);
        }
        #endregion
    }
}
