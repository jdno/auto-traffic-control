
/* The authors of auto-traffic-control release this file under either Apache v2.0 or MIT license terms. */

package com.example;


import atc.v1.Game;
import atc.v1.Game.StartGameRequest;
import atc.v1.Game.StartGameResponse;
import atc.v1.GameServiceGrpc;
import atc.v1.GameServiceGrpc.GameServiceBlockingStub;
import atc.v1.GameServiceGrpc.GameServiceFutureStub;

import com.google.common.util.concurrent.FutureCallback;
import com.google.common.util.concurrent.Futures;
import com.google.common.util.concurrent.ListenableFuture;
import com.google.common.util.concurrent.MoreExecutors;

import io.grpc.Channel;
import io.grpc.ManagedChannelBuilder;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.Future;


/**
Demonstrates connecting and then starting Auto-Traffic-Control.
*/
public class AtcExample
{

	/** Connects to localhost port 4747 and starts game */
	public static void main(
			String[] args
	) {
		AtcExample game = new AtcExample( "localhost", 4747 );
		boolean whetherAsynchronous = true;
		game.startGame( whetherAsynchronous );
	}


	private Channel channel;
	private GameServiceBlockingStub syncingGameServiceStub;
	private GameServiceFutureStub asyncGameServiceStub;


	public AtcExample(
			String host, int port
	) {
		this( ManagedChannelBuilder.forAddress( host, port ).usePlaintext() );
	}


	public AtcExample(
			ManagedChannelBuilder<?> channelBuilder
	) {
		channel = channelBuilder.build();
		syncingGameServiceStub = GameServiceGrpc.newBlockingStub( channel );
		asyncGameServiceStub = GameServiceGrpc.newFutureStub( channel );
	}


	public boolean startGame(
			boolean asynchronous
	) {
		final String successMessage = "The game is afoot";
		StartGameRequest startsGame = StartGameRequest.newBuilder().build();
		if ( asynchronous )
		{
			int ticksToWait = 1;
			ListenableFuture<StartGameResponse> maybeStarted
					= asyncGameServiceStub.startGame( startsGame );
			final CountDownLatch waitForFuture = new CountDownLatch( ticksToWait );
			Futures.addCallback(
				maybeStarted,
				new FutureCallback<StartGameResponse>() {
					@Override
					public void onSuccess( StartGameResponse result )
					{
						System.out.println( successMessage +" "+ result );
						waitForFuture.countDown();
					}
					//
					@Override
					public void onFailure(Throwable t)
					{
						throw new Error( t );
					}
				},
				MoreExecutors.directExecutor() );
			try
			{
				waitForFuture.await();
			}
			catch ( InterruptedException ie )
			{
				ie.printStackTrace();
				return false;
			}
			return true; // ¶ not actually finished, given the asynchronous communication
		}
		else
		{
			syncingGameServiceStub.startGame( startsGame );
			System.out.println( successMessage );
			return true;
		}
	}


}


















