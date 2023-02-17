namespace LooCast.System.Identification
{
    public interface IGameObjectIdentifiable : IObjectIdentifiable, IUnityInstanceIdentifiable
    {
        #region Properties
        IGameObjectIdentifier GameObjectIdentifier { get; }
        #endregion
    }
}
