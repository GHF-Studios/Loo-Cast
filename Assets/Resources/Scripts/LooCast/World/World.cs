using UnityEngine;

namespace LooCast.World
{
    public class World : MonoBehaviour
    {
        //Square Chunk's side length, preferably being a power of 2
        private int chunkSize = 64;

        //The seed from which everything in the world is procedurally generated, which will create an exact duplicate if used again
        private int seed = 0;

        
    }
}