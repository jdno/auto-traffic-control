package com.example;

import atc.v1.Atc.GetVersionRequest;
import atc.v1.Atc.GetVersionResponse;
import atc.v1.Atc.Version;
import atc.v1.AtcServiceGrpc;
import atc.v1.AtcServiceGrpc.AtcServiceBlockingStub;
import atc.v1.AtcServiceGrpc.AtcServiceFutureStub;
import com.google.common.util.concurrent.FutureCallback;
import com.google.common.util.concurrent.Futures;
import com.google.common.util.concurrent.ListenableFuture;
import com.google.common.util.concurrent.MoreExecutors;
import io.grpc.Channel;
import io.grpc.ManagedChannelBuilder;

import java.util.concurrent.CountDownLatch;

/**
 * Demonstrates connecting and then starting Auto-Traffic-Control.
 */
public class AtcExample {

    /**
     * Connects to localhost port 4747 and starts game
     */
    public static void main(String[] args) {
        AtcExample game = new AtcExample("localhost", 4747);
        boolean whetherAsynchronous = true;
        game.getVersion(whetherAsynchronous);
    }

    private Channel channel;
    private AtcServiceBlockingStub syncingAtcServiceStub;
    private AtcServiceFutureStub asyncAtcServiceStub;

    public AtcExample(String host, int port) {
        this(ManagedChannelBuilder.forAddress(host, port).usePlaintext());
    }

    public AtcExample(ManagedChannelBuilder<?> channelBuilder) {
        channel = channelBuilder.build();
        syncingAtcServiceStub = AtcServiceGrpc.newBlockingStub(channel);
        asyncAtcServiceStub = AtcServiceGrpc.newFutureStub(channel);
    }

    public boolean getVersion(boolean asynchronous) {
        GetVersionRequest versionRequest = GetVersionRequest.newBuilder().build();

        if (asynchronous) {
            ListenableFuture<GetVersionResponse> maybeVersion = asyncAtcServiceStub.getVersion(versionRequest);

            int ticksToWait = 1;
            final CountDownLatch waitForFuture = new CountDownLatch(ticksToWait);

            Futures.addCallback(maybeVersion, new FutureCallback<GetVersionResponse>() {
                @Override
                public void onSuccess(GetVersionResponse result) {
                    printVersion(result.getVersion());
                    waitForFuture.countDown();
                }

                @Override
                public void onFailure(Throwable t) {
                    throw new Error(t);
                }
            }, MoreExecutors.directExecutor());
            try {
                waitForFuture.await();
            } catch (InterruptedException ie) {
                ie.printStackTrace();
                return false;
            }
            return true; // not actually finished, given the asynchronous communication
        } else {
            GetVersionResponse result = syncingAtcServiceStub.getVersion(versionRequest);
            printVersion(result.getVersion());

            return true;
        }
    }

    private static void printVersion(Version version) {
        System.out.println("Auto Traffic Control is running version " + version.getMajor() + "." + version.getMinor() + "." + version.getPatch());
    }
}
