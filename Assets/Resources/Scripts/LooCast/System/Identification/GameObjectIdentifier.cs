using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public struct GameObjectIdentifier : IIdentifier
    {
        #region Properties
        public string GUSID
        {
            get
            {
                return $"{ContainingTypeIdentifier}({GameObjectInstanceGUID})";
            }
        }
        public TypeIdentifier ContainingTypeIdentifier => containingTypeIdentifier;
        public Guid GameObjectInstanceGUID => gameObjectinstanceGUID;
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier containingTypeIdentifier;
        [SerializeField] private Guid gameObjectinstanceGUID;
        #endregion

        #region Constructors
        public GameObjectIdentifier(TypeIdentifier containingTypeIdentifier, Guid gameObjectinstanceGUID)
        {
            this.containingTypeIdentifier = containingTypeIdentifier;
            this.gameObjectinstanceGUID = gameObjectinstanceGUID;
        }
        #endregion

        #region Static Methods
        public static bool TryParse(string gusid, out GameObjectIdentifier? gameObjectIdentifier)
        {
            gameObjectIdentifier = null;

            string[] parts = gusid.Split(new char[] { '(', ')' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string containingTypeIdentifierString = parts[0];
            string gameObjectInstanceGUIDString = parts[1];

            if (!TypeIdentifier.TryParse(containingTypeIdentifierString, out TypeIdentifier? containingTypeIdentifier))
            {
                return false;
            }

            if (!Guid.TryParse(gameObjectInstanceGUIDString, out Guid gameObjectinstanceGUID))
            {
                return false;
            }

            gameObjectIdentifier = new GameObjectIdentifier(containingTypeIdentifier.Value, gameObjectinstanceGUID);
            return true;
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return GUSID;
        }
        
        public override bool Equals(object obj)
        {
            if (obj is GameObjectIdentifier)
            {
                return Equals((GameObjectIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(GameObjectIdentifier otherGameObjectIdentifier)
        {
            return otherGameObjectIdentifier.GUSID.Equals(this.GUSID);
        }

        public override int GetHashCode()
        {
            return GUSID.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(GameObjectIdentifier gameObjectIdentifier1, GameObjectIdentifier gameObjectIdentifier2)
        {
            return gameObjectIdentifier1.Equals(gameObjectIdentifier2);
        }

        public static bool operator !=(GameObjectIdentifier gameObjectIdentifier1, GameObjectIdentifier gameObjectIdentifier2)
        {
            return !gameObjectIdentifier1.Equals(gameObjectIdentifier2);
        }

        public static implicit operator string(GameObjectIdentifier gameObjectIdentifier)
        {
            return gameObjectIdentifier.GUSID;
        }

        public static implicit operator GameObjectIdentifier(string gusid)
        {
            if (TryParse(gusid, out GameObjectIdentifier? gameObjectIdentifier))
            {
                return gameObjectIdentifier.Value;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid GUSID.");
            }
        }
        #endregion
    }
}
