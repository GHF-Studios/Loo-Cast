namespace LooCast.System.Identification
{
    public interface IGameObjectIdentifiable : IUnityInstanceIdentifiable
    {
        #region Properties
        IGameObjectIdentifier GameObjectIdentifier { get; }
        #endregion
    }
}
